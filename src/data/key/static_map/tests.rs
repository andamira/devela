// devela::data::key::static_map::tests
//
//!
//
// IMPROVE: make copy methods const, use const_assert

use crate::define_static_map;

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod v_non_copy {
    use crate::{StaticMapEntry, String, ToString};

    super::define_static_map![const TestMapU8, KEY: u8];

    #[test]
    fn get_ref() {
        let mut map = TestMapU8::<u8, u32, 4>::default();
        map.insert(1, 100).unwrap();
        assert_eq!(map.get_ref(1), Some(&100));
        assert_eq!(map.get_ref(2), None);
    }
    #[test]
    fn get_mut() {
        let mut map = TestMapU8::<u8, String, 4>::default();
        map.insert_move(1, "Hello".to_string()).unwrap();
        if let Some(v) = map.get_mut(1) {
            v.push_str(", World!");
        }
        assert_eq!(map.get_ref(1), Some(&"Hello, World!".to_string()));
    }

    #[test]
    fn entry_occupied() {
        let mut map = TestMapU8::<u8, u32, 4>::default();
        map.insert(1, 100).unwrap();
        match map.entry(1) {
            StaticMapEntry::Occupied(v) => {
                *v = 200; // Modify in place
            }
            _ => panic!("Expected occupied entry"),
        }
        assert_eq!(map.get_ref(1), Some(&200));
    }
    #[test]
    fn entry_vacant() {
        let mut map = TestMapU8::<u8, u32, 4>::default();
        match map.entry(1) {
            StaticMapEntry::Vacant(idx) => {
                map.keys[idx] = 1;
                map.values[idx] = 100;
            }
            _ => panic!("Expected vacant entry"),
        }
        assert_eq!(map.get_ref(1), Some(&100));
    }

    #[test]
    fn insert_move() {
        let mut map = TestMapU8::<u8, String, 4>::default();
        map.insert_move(1, "Hello".to_string()).unwrap();
        assert_eq!(map.get_ref(1), Some(&"Hello".to_string()));
    }

    #[test]
    fn replace() {
        let mut map = TestMapU8::<u8, String, 4>::default();
        map.insert_move(1, "Hello".to_string()).unwrap();
        assert_eq!(map.replace(1, "Replaced".to_string()), Some("Hello".to_string()));
        assert_eq!(map.get_ref(1), None);
    }
    #[test]
    fn replace_default() {
        let mut map = TestMapU8::<u8, String, 4>::default();
        map.insert_move(1, "Hello".to_string()).unwrap();
        assert_eq!(map.replace_default(1), Some("Hello".to_string()));
        assert_eq!(map.get_ref(1), None);
    }
    #[test]
    fn replace_with() {
        let mut map = TestMapU8::<u8, String, 4>::default();
        map.insert_move(1, "Hello".to_string()).unwrap();
        assert_eq!(map.replace_with(1, || "Replaced".to_string()), Some("Hello".to_string()));
        assert_eq!(map.get_ref(1), None);
    }
    #[test]
    fn type_id() {
        use crate::{Hash, Hasher, HasherFx, TypeId};

        /// Hashes a `TypeId` into a `u64` using `HasherFx`.
        fn hash_type_id<T: 'static>() -> u64 {
            let mut hasher = HasherFx::<u64>::new();
            TypeId::of::<T>().hash(&mut hasher);
            hasher.finish()
        }
        struct Empty;
        struct Tomb;

        super::define_static_map![TypeIdMapU8, KEY:u64,
            EMPTY:hash_type_id::<Empty>(),
            TOMB:hash_type_id::<Tomb>()];
        let mut map: TypeIdMapU8<u64, char, 4> = TypeIdMapU8::new();

        // Insert some type-based keys
        let type_a = hash_type_id::<i32>();
        let type_b = hash_type_id::<f64>();

        assert!(map.insert(type_a, 'A').is_ok());
        assert!(map.insert(type_b, 'B').is_ok());

        // Retrieve values
        assert_eq!(map.get_ref(type_a), Some(&'A'));
        assert_eq!(map.get_ref(type_b), Some(&'B'));

        // Ensure a non-existent key returns `None`
        let unknown_type = hash_type_id::<bool>();
        assert_eq!(map.get_ref(unknown_type), None);
    }
}

mod k_u8 {
    super::define_static_map![const TestMapU8, KEY:u8];

    #[test]
    const fn map_custom_empty_tomb() {
        super::define_static_map![const TestMapU8Custom, KEY: u8, EMPTY: 1, TOMB: 66];
        let mut _map = TestMapU8Custom::<u8, u32, 4>::new();
    }
    #[test]
    fn map_insert_get() {
        let mut map = TestMapU8::<u8, u32, 4>::new();
        assert_eq!(map.insert(1, 100), Ok(()));
        assert_eq!(map.get(1), Some(100));
        assert_eq!(map.get(2), None);
    }
    #[test]
    fn map_insert_overwrite() {
        let mut map = TestMapU8::<u8, u32, 4>::new();
        assert_eq!(map.insert(1, 100), Ok(()));
        assert_eq!(map.insert(1, 200), Ok(())); // overwrite
        assert_eq!(map.get(1), Some(200));
    }
    #[test]
    fn map_remove() {
        let mut map = TestMapU8::<u8, u32, 4>::new();
        assert_eq!(map.insert(1, 100), Ok(()));
        assert_eq!(map.remove(1), true);
        assert_eq!(map.get(1), None);
        assert_eq!(map.remove(1), false); // already removed
    }
    #[test]
    fn should_rebuild() {
        let mut map = TestMapU8::<u8, u32, 8>::new();
        assert!(!map.should_rebuild());
        assert_eq!(map.insert(1, 100), Ok(()));
        assert_eq!(map.insert(2, 200), Ok(()));
        assert_eq!(map.insert(3, 300), Ok(()));
        assert_eq!(map.insert(4, 400), Ok(()));
        assert!(!map.should_rebuild());
        // remove some values but not enough to trigger a rebuild
        assert_eq!(map.remove(2), true);
        assert_eq!(map.remove(4), true);
        assert!(!map.should_rebuild());
        // remove more values to pass the threshold
        assert_eq!(map.remove(1), true);
        assert_eq!(map.remove(3), true);
        assert!(map.should_rebuild()); // now it should require a rebuild
    }
    #[test]
    fn map_collision() {
        let mut map = TestMapU8::<u8, u32, 2>::new();
        assert_eq!(map.hash_index(1), map.hash_index(3)); // make sure keys collide
        assert_eq!(map.insert(1, 100), Ok(()));
        assert_eq!(map.insert(3, 200), Ok(())); // collides with 1, probes forward
        assert_eq!(map.get(1), Some(100));
        assert_eq!(map.get(3), Some(200));
    }
}

#[rustfmt::skip]
mod k_u8_panicking {
    super::define_static_map![const TestMapU8, KEY:u8];

    #[allow(unused)] type M = TestMapU8::<u8, u32, 4>;
    #[test] #[cfg(debug_assertions)] #[should_panic(expected = "Key cannot be `EMPTY` marker")]
    fn insert_empty_key() { let mut m = M::new(); let _ = m.insert(M::EMPTY, 42); }
    #[test] #[cfg(debug_assertions)] #[should_panic(expected = "Key cannot be `TOMB` marker")]
    fn insert_tombstone_key() { let mut m = M::new(); let _ = m.insert(M::TOMB, 42); }
    #[test] #[cfg(debug_assertions)] #[should_panic(expected = "Key cannot be `EMPTY` marker")]
    fn get_empty_key() { let m = M::new(); let _ = m.get(M::EMPTY); }
    #[test] #[cfg(debug_assertions)] #[should_panic(expected = "Key cannot be `TOMB` marker")]
    fn get_tombstone_key() { let m = M::new(); let _ = m.get(M::TOMB); }
    #[test] #[cfg(debug_assertions)] #[should_panic(expected = "Key cannot be `EMPTY` marker")]
    fn remove_empty_key() { let mut m = M::new(); let _ = m.remove(M::EMPTY); }
    #[test] #[cfg(debug_assertions)] #[should_panic(expected = "Key cannot be `TOMB` marker")]
    fn remove_tombstone_key() { let mut m = M::new(); let _ = m.remove(M::TOMB); }
}

mod k_f32 {
    super::define_static_map![const TestMapF32, KEY:f32];

    #[test]
    fn map_custom_empty_tomb() {
        super::define_static_map![const TestMapF32Custom,
            KEY: f32, EMPTY: f32::NEG_INFINITY, TOMB: f32::INFINITY];
        let mut _map = TestMapF32Custom::<f32, u32, 4>::new();
    }
    #[test]
    fn map_insert_get() {
        let mut map = TestMapF32::<f32, u32, 4>::new();
        assert_eq!(map.insert(0.23, 100), Ok(()));
        assert_eq!(map.get(0.23), Some(100));
        assert_eq!(map.get(1.64), None);
    }
    #[test]
    fn map_insert_overwrite() {
        let mut map = TestMapF32::<f32, u32, 4>::new();
        assert_eq!(map.insert(0.51, 100), Ok(()));
        assert_eq!(map.insert(0.51, 200), Ok(())); // overwrite
        assert_eq!(map.get(0.51), Some(200));
    }
    #[test]
    fn map_remove() {
        let mut map = TestMapF32::<f32, u32, 4>::new();
        assert_eq!(map.insert(0.33, 100), Ok(()));
        assert_eq!(map.remove(0.33), true);
        assert_eq!(map.get(0.33), None);
        assert_eq!(map.remove(0.33), false); // already removed
    }
    #[test]
    fn map_collision() {
        let mut map = TestMapF32::<f32, u32, 2>::new();
        assert_eq!(map.hash_index(0.3), map.hash_index(1.3)); // make sure keys collide
        assert_eq!(map.insert(0.3, 100), Ok(()));
        assert_eq!(map.insert(1.3, 200), Ok(())); // collides with 1, probes forward
        assert_eq!(map.get(0.3), Some(100));
        assert_eq!(map.get(1.3), Some(200));
    }
}

mod k_char {
    super::define_static_map![const TestMapChar, KEY:char];

    #[test]
    fn map_custom_empty_tomb() {
        super::define_static_map![const TestMapCharCustom, KEY: char, EMPTY: '\0', TOMB: 'Ŧ'];
        let mut map = TestMapCharCustom::<char, u32, 4>::new();
        assert_eq!(map.insert('a', 100), Ok(()));
    }
    #[test]
    fn map_insert_get() {
        let mut map = TestMapChar::<char, u32, 4>::new();
        assert_eq!(map.insert('a', 100), Ok(()));
        assert_eq!(map.get('a'), Some(100));
        assert_eq!(map.get('b'), None);
    }
    #[test]
    fn map_insert_overwrite() {
        let mut map = TestMapChar::<char, u32, 4>::new();
        assert_eq!(map.insert('€', 100), Ok(()));
        assert_eq!(map.insert('€', 200), Ok(())); // overwrite
        assert_eq!(map.get('€'), Some(200));
    }
    #[test]
    fn map_remove() {
        let mut map = TestMapChar::<char, u32, 4>::new();
        assert_eq!(map.insert('x', 100), Ok(()));
        assert_eq!(map.remove('x'), true);
        assert_eq!(map.get('x'), None);
        assert_eq!(map.remove('x'), false); // already removed
    }
    #[test]
    fn map_collision() {
        let mut map = TestMapChar::<char, u32, 2>::new();
        assert_eq!(map.hash_index('a'), map.hash_index('c')); // make sure keys collide
        assert_eq!(map.insert('a', 100), Ok(()));
        assert_eq!(map.insert('c', 200), Ok(())); // collides with 'a', probes forward
        assert_eq!(map.get('a'), Some(100));
        assert_eq!(map.get('c'), Some(200));
    }
}
