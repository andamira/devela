//
//! Defines [`Search`] and [`SearchNode`].
//

use crate::{NotEnoughSpace, is};

#[doc = crate::_tags!(data)]
/// A discovered state and its incoming transition.
#[doc = crate::_doc_meta!{
    location("data/access/search", struct SearchNode),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SearchNode<S, A> {
    /// The discovered state.
    pub state: S,
    /// The parent record index and the action from it, or `None` at the start.
    pub via: Option<(usize, A)>,
}

#[doc = crate::_tags!(data namespace)]
/// Search over caller-defined state spaces.
#[doc = crate::_doc_meta!{
    location("data/access/search", struct Search),
    test_size_of(Search = 0),
}]
#[derive(Debug)]
pub struct Search;

impl Search {
    /// Finds a goal using breadth-first search with caller-provided storage.
    ///
    /// `successors` yields `(state, action)` pairs in exploration order.
    /// Its iterator cannot borrow the input state. The goal and successor
    /// semantics must remain fixed during the search.
    /// States are deduplicated by equality, so equal states must have equivalent
    /// goal and successor semantics. Each transition has unit cost.
    /// The first discovered goal has a shortest path in number of transitions;
    /// ties follow successor order.
    ///
    /// `nodes` is cleared on entry. On return, including insufficient space,
    /// it contains a dense prefix of discovered records followed by `None`.
    /// Every non-root record refers to an earlier record through `via`.
    ///
    /// Returns:
    /// - `Ok(Some(index))` when a goal is found. Follow `via` back to the root.
    /// - `Ok(None)` when the reachable state space is exhausted without a goal.
    /// - `Err(NotEnoughSpace)` when a new state cannot be retained.
    ///   This does not establish that the goal is unreachable.
    ///
    /// The caller must provide a finite successor iterator for each state.
    /// No allocation is performed by the search itself. Membership is checked
    /// linearly over discovered states, favoring small bounded state spaces
    /// over large graphs. Each candidate costs at most O(discovered states)
    /// equality comparisons, and each retained state occupies one record.
    ///
    /// # Example
    /// ```
    /// use devela::Search;
    /// let mut nodes = [None; 4];
    /// let goal = Search::bfs(0u8, |&s| s == 3, |&s| match s {
    ///     0 => &[(1, 'a'), (2, 'b')][..],
    ///     1 => &[(3, 'c')][..],
    ///     _ => &[][..],
    /// }.iter().copied(), &mut nodes).unwrap().unwrap();
    /// let (parent, action) = nodes[goal].unwrap().via.unwrap();
    /// assert_eq!((nodes[parent].unwrap().state, action), (1, 'c'));
    /// ```
    pub fn bfs<S, A, I>(
        start: S,
        mut is_goal: impl FnMut(&S) -> bool,
        mut successors: impl FnMut(&S) -> I,
        nodes: &mut [Option<SearchNode<S, A>>],
    ) -> Result<Option<usize>, NotEnoughSpace>
    where
        S: Eq,
        I: IntoIterator<Item = (S, A)>,
    {
        is! { nodes.is_empty(), return Err(NotEnoughSpace(Some(1))) }
        for slot in nodes.iter_mut() {
            *slot = None;
        }
        let found = is_goal(&start);
        nodes[0] = Some(SearchNode { state: start, via: None });
        is! { found, return Ok(Some(0)) }
        // The retained records double as the FIFO frontier: head..len.
        let (mut head, mut len) = (0, 1);
        while head < len {
            for (state, action) in successors(&nodes[head].as_ref().unwrap().state) {
                is! { nodes[..len].iter().flatten().any(|node| node.state == state), continue }
                is! { len == nodes.len(), return Err(NotEnoughSpace(Some(1))) }
                let found = is_goal(&state);
                nodes[len] = Some(SearchNode { state, via: Some((head, action)) });
                is! { found, return Ok(Some(len)) }
                len += 1;
            }
            head += 1;
        }
        Ok(None)
    }
}
