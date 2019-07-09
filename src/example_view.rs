use std::{
    collections::HashMap,
    sync::{mpsc::Receiver, Arc},
    thread,
};

use pvm_plugins::views::{DBTr, View, ViewInst, ViewParams, ViewParamsExt};

#[derive(Debug)]
pub struct ExampleView {
    id: usize,
}

impl View for ExampleView {
    fn new(id: usize) -> ExampleView {
        ExampleView { id }
    }
    fn id(&self) -> usize {
        self.id
    }
    fn name(&self) -> &'static str {
        "ExampleView"
    }
    fn desc(&self) -> &'static str {
        "View example."
    }
    fn params(&self) -> HashMap<&'static str, &'static str> {
        let mut params = HashMap::new();
        params.insert("example", "example parameter");
        params
    }
    fn create(
        &self,
        id: usize,
        params: ViewParams,
        stream: Receiver<Arc<DBTr>>,
    ) -> ViewInst {
        let _path = params.get_or_def("example", "example_default_value");

        let thr = thread::Builder::new()
            .name("ExampleView".to_string())
            .spawn(move || {
                for _tr in stream { /* Do Something */ }
            })
            .unwrap();
        ViewInst {
            id,
            vtype: self.id,
            params,
            handle: thr,
        }
    }
}
