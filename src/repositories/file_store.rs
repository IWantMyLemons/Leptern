use leptos::prelude::*;
use leptos::wasm_bindgen::prelude::*;
use web_sys::js_sys::ArrayBuffer;
use web_sys::{Event, File, FileReader, IdbDatabase, IdbTransactionMode};

/// Saves a file to the browser's indexeddb with it's filename as a key and array of bytes as a value
pub fn save_file_to_idb(db: IdbDatabase, file: File) {
    let filename = file.name();
    let reader = FileReader::new().unwrap();
    let on_load_end = Closure::<dyn FnMut(_)>::new(move |ev: Event| {
        let reader = event_target::<FileReader>(&ev);
        match reader.result() {
            Ok(result) => {
                let array: ArrayBuffer = ArrayBuffer::from(result);

                db.transaction_with_str_and_mode("files", IdbTransactionMode::Readwrite)
                    .and_then(|transaction| transaction.object_store("files"))
                    .and_then(|store| store.put_with_key(&array, &filename.clone().into()))
                    .unwrap();

                log::info!("Saved {filename} to database :D");
            }
            Err(err) => {
                log::error!("{err:?}");
            }
        };
    });
    reader.set_onloadend(Some(on_load_end.as_ref().unchecked_ref()));
    reader.read_as_array_buffer(&file).unwrap();
    on_load_end.forget();
}

// Loads a file from the browser's indexeddb
// pub fn load_file_from_idb(filename: String) {
//     todo!()
// }
