// devela::work::reexports::task
//
//! Reexported items.
//

use crate::code::reexport;

reexport! { rust: core::task, local_module: "work",
    doc: "The context of an asynchronous task.",
    @Context as TaskContext
}
reexport! { rust: core::task, local_module: "work",
    doc: "Allows the implementor of a task executor to create a [`TaskWaker`].",
    @RawWaker as TaskRawWaker
}
reexport! { rust: core::task, local_module: "work",
    doc: "A virtual function pointer table that specifies the behavior of a [`TaskRawWaker`].",
    @RawWakerVTable as TaskRawWakerVTable
}
reexport! { rust: core::task, local_module: "work",
    doc: "A handle for waking up a task by notifying its executor that it is ready to be run.",
    @Waker as TaskWaker
}
reexport! { rust: core::task, local_module: "work",
    doc: "Indicates whether a value is available or if the current task has been scheduled
        to receive a wakeup instead.",
    @Poll as TaskPoll
}
reexport! { rust: core::task, local_module: "work",
    doc: "Extracts the successful type of a [`TaskPoll<T>`].",
    @ready as task_ready
}
reexport! { rust: "alloc"|_alloc::task, local_module: "work",
    doc: "The implementation of waking a task on an executor.",
    @Wake as TaskWake
}
