extern crate serde;
extern crate restful;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate restful_derive;
#[macro_use] extern crate error_chain;

pub mod errors {
	error_chain! {
		foreign_links {
			Request(::restful::Error);
			Io(::std::io::Error);
		}
	}
}

mod models;

use std::path::Path;
use std::collections::HashMap;

use restful::{RestApi, Response, Form};

pub use self::models::*;
use self::errors::Result;

#[derive(Debug, Clone)]
pub struct CuckooApi {
	api: RestApi,
}

impl CuckooApi {
    pub fn new(base_url: &str) -> CuckooApi {
        CuckooApi {
            api: RestApi::new(base_url)
        }
    }

    pub fn get_tasks(&self) -> Result<Vec<CuckooTask>> {
        Ok(CuckooTasksResource::get(&self.api).map(|list| list.tasks)?)
    }

    pub fn get_task(&self, task_id: i32) -> Result<CuckooTask> {
        Ok(CuckooTaskResource::get(&self.api, task_id).map(|detail| detail.task)?)
    }

    pub fn create_task<U: AsRef<Path>>(&self, file_path: U, timeout: u32, options: &HashMap<&str, &str>) -> Result<CuckooTaskCreateResource> {
        let options = options.iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .fold(String::new(), |opts, opt| format!("{},{}", opts, opt));
        let form = Form::new()
            .text("timeout", timeout.to_string())
            .text("enforce_timeout", "true")
            .text("options", options)
            .file("file", file_path)?;
        Ok(CuckooTaskCreateResource::post_multipart(&self.api, form)?)
    }

    pub fn get_pcap(&self, task_id: i32) -> Result<Response> {
        Ok(self.api.get(&format!("/pcap/get/{}", task_id))?)
    }
}
