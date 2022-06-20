use atomic_counter::RelaxedCounter;
use idmap::OrderedIdMap;
use lazy_static::lazy_static;
use viemo::gen_type_constructor;
use viemo::store::Store;
use viemo::versioned_cell::VersionedCell;

pub struct TodoApp<'store> {
    pub todos: VersionedCell<'store, OrderedIdMap<usize, VersionedCell<'store, TodoItem>>>,
}

gen_type_constructor!(TodoApp, pub TodoAppTC);

pub struct TodoItem {
    pub note: String,
    pub complete: bool,
}

lazy_static! {
    pub static ref APP_DATA: Store<TodoAppTC> = Store::initialize(|cx| {
        TodoApp {
            todos: VersionedCell::new(cx, OrderedIdMap::new()),
        }
    });
    pub static ref TODO_ID_PROVIDER: RelaxedCounter = RelaxedCounter::new(0);
}
