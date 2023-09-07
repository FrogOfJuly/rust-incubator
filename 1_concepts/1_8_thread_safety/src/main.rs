use std::{marker::PhantomData, rc::Rc, sync::{MutexGuard, Mutex}};

//T : Send - T and &mut T can be passed between threads
//T : Sync - &T can be passed between threads

// T : Sync <=> &T : Send

struct NotSyncNotSend{
    p : PhantomData<Rc<Vec<i32>>>
}

struct SyncAndSend{
    p : PhantomData<Vec<i32>>
}

struct NotSend<'a>{
    p : PhantomData<MutexGuard<'a, i32>>
}

struct NotSync{
    // p : Box<dyn Default + !Send> ??
}



fn sync_send<T : Sync + Send>(_x : T){}



fn main() {
    // sync_send(NotSyncNotSend{ p: PhantomData });
    sync_send(SyncAndSend{ p: PhantomData });
    // sync_send(NotSend{ p: PhantomData });
    sync_send(NotSync{ p: PhantomData });

}
