// devela::run::driver::_helper
//
//! Defines `_run_driver_step_run_frame_body!`.
//

/// Expands the shared core of [`RunDriver::step_frame`] and [`RunDriver::run_frame`].
///
/// This exists to avoid repeating the same higher-ranked `RunRender`/`RunPresent`
/// orchestration body in two methods, which currently helps sidestep compiler
/// limitations around reusing the generic frame-step path through a normal helper.
///
/// The macro performs:
/// - phase validation,
/// - event collection,
/// - logical stepping,
/// - frame construction,
/// - rendering,
/// - presentation,
/// - and runtime progression update.
///
/// The optional trailing expression is executed when the app returns
/// [`RunControl::Stop`], allowing callers such as `run_frame` to `break`.
macro_rules! _run_driver_step_run_frame_body {
    // with phase check + scene projection
    (@check
     $runtime:expr, $backend:expr, $app:expr, $renderer:expr, $presenter:expr,
     scene_of($scene_of:expr), $events:ident, $control:ident,
     $on_continue:expr, $on_stop:expr $(,$break:expr)?) => {
        if !$runtime.can_advance() {
            return Err(RunDriverFrameError::InvalidPhase($runtime.phase()));
        }
        $crate::_run_driver_step_run_frame_body!(@body
            $runtime, $backend, $app, $renderer, $presenter,
            $scene_of(&*$app), $events, $control,
            $on_continue, $on_stop $(,$break)?
        );
    };
    // with phase check + direct scene
    (@check
     $runtime:expr, $backend:expr, $app:expr, $renderer:expr, $presenter:expr,
     scene($scene:expr), $events:ident, $control:ident,
     $on_continue:expr, $on_stop:expr $(,$break:expr)?) => {
        if !$runtime.can_advance() {
            return Err(RunDriverFrameError::InvalidPhase($runtime.phase()));
        }
        $crate::_run_driver_step_run_frame_body!(@body
            $runtime, $backend, $app, $renderer, $presenter,
            $scene, $events, $control,
            $on_continue, $on_stop $(,$break)?
        );
    };

    // without phase check + scene projection
    ($runtime:expr, $backend:expr, $app:expr, $renderer:expr, $presenter:expr,
     scene_of($scene_of:expr), $events:ident, $control:ident,
     $on_continue:expr, $on_stop:expr $(,$break:expr)?) => {
        $crate::_run_driver_step_run_frame_body!(@body
            $runtime, $backend, $app, $renderer, $presenter,
            $scene_of(&*$app), $events, $control,
            $on_continue, $on_stop $(,$break)?
        );
    };
    // without phase check + direct scene
    ($runtime:expr, $backend:expr, $app:expr, $renderer:expr, $presenter:expr,
     scene($scene:expr), $events:ident, $control:ident,
     $on_continue:expr, $on_stop:expr $(,$break:expr)?) => {
        $crate::_run_driver_step_run_frame_body!(@body
            $runtime, $backend, $app, $renderer, $presenter,
            $scene, $events, $control,
            $on_continue, $on_stop $(,$break)?
        );
    };

    // shared body
    (@body
     $runtime:expr, $backend:expr, $app:expr, $renderer:expr, $presenter:expr,
     $scene_or_projection:expr, $events:ident, $control:ident,
     $on_continue:expr, $on_stop:expr $(,$break:expr)?) => {
        // 1. Gather normalized backend events into caller-provided storage.
        let written = $backend.collect_events($events)
            .map_err(RunDriverFrameError::Backend)?;
        let $events = &$events[..written];

        // 2. Build the logical step snapshot.
        let step = RunStep::new($runtime.tick(), $runtime.phase(), $events);

        // 3. Advance app logic first, so rendering observes post-step state.
        let $control = $app.run_step(step)
            .map_err(RunDriverFrameError::App)?;

        // 4. Build one backend-facing frame snapshot shared by render and present.
        let mut frame = $backend.frame($runtime.tick(), $runtime.phase(), $events);

        // 5. Render the current scene or app-facing projection.
        let scene = $scene_or_projection;
        let artifact = $renderer.run_render(&mut frame, scene)
            .map_err(RunDriverFrameError::Render)?;

        // 6. Finalize or expose the rendered artifact.
        $presenter.run_present(&mut frame, artifact)
            .map_err(RunDriverFrameError::Present)?;

        // 7. Update logical progression.
        match $control {
            RunControl::Continue => { $on_continue; }
            RunControl::Stop => {
                $on_stop;
                $($break)?
            }
        }
    };
}
pub(crate) use _run_driver_step_run_frame_body;
