use pyo3::exceptions::PyOSError;
use pyo3::types::{PyTuple, PyDict};
use pyo3::prelude::*;
use sonic_channel::*;

#[pyclass(name = "SearchChannel")]
#[pyo3(text_signature = "(self, addr, passwd)")]
struct PySearchChannel {
    channel: SearchChannel,
}

impl PySearchChannel {
    /// Query objects in database.
    fn query_impl(
        &self,
        collection: &str,
        bucket: Option<&str>,
        terms: String,
        lang: Option<&str>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> PyResult<Vec<String>> {
        let dest = if let Some(bucket) = bucket {
            Dest::col_buc(collection, bucket)
        } else {
            Dest::col(collection)
        };

        let mut request = QueryRequest {
            dest,
            terms,
            limit,
            offset,
            lang: None,
        };

        if let Some(lang) = lang {
            request.lang = Lang::from_code(lang);
        }

        self.channel
            .query(request)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    /// Suggest auto-completes words.
    fn suggest_impl(
        &self,
        collection: &str,
        bucket: Option<&str>,
        word: String,
        limit: Option<usize>,
    ) -> PyResult<Vec<String>> {
        let dest = if let Some(bucket) = bucket {
            Dest::col_buc(collection, bucket)
        } else {
            Dest::col(collection)
        };
        let request = SuggestRequest { dest, word, limit };
        self.channel
            .suggest(request)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }
}

#[pymethods]
impl PySearchChannel {
    #[new]
    fn new(addr: &str, password: &str) -> PyResult<Self> {
        let channel =
            SearchChannel::start(addr, password).map_err(|e| PyOSError::new_err(e.to_string()))?;
        Ok(PySearchChannel { channel })
    }

    /// Ping server.
    #[pyo3(text_signature = "(self)")]
    fn ping(&self) -> PyResult<()> {
        self.channel
            .ping()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    /// Stop connection.
    #[pyo3(text_signature = "(self)")]
    fn quit(&self) -> PyResult<()> {
        self.channel
            .quit()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    /// Query objects in database.
    /// The function can be called in two ways:
    ///     `query(self, collection, bucket, terms, lang=None, limit=None, offset=None)`
    ///     `query(self, collection, terms, lang=None, limit=None, offset=None)`
    /// where `bucket` is optional.
    #[pyo3(text_signature = "(self, collection, *args, lang=None, limit=None, offset=None, **kwargs)")]
    fn query(
        &self,
        collection: &str,
        args: &PyTuple,
        lang: Option<&str>,
        limit: Option<usize>,
        offset: Option<usize>,
        kwargs: Option<&PyDict>,
    ) -> PyResult<Vec<String>> {
        let bucket = kwargs.and_then(|d| d.get_item("bucket"));
        let terms = kwargs.and_then(|d| d.get_item("terms"));

        match (bucket, terms, args.len()) {
            (Some(bucket), Some(terms), 0) => {
                let bucket = bucket.extract()?;
                let terms = terms.extract()?;
                self.query_impl(collection, Some(bucket), terms, lang, limit, offset)
            }
            (None, Some(terms), 0) => {
                let bucket = None;
                let terms = terms.extract()?;
                self.query_impl(collection, bucket, terms, lang, limit, offset)
            }
            (None, None, 1) => {
                let bucket = None;
                let terms = args.get_item(0)?.extract()?;
                self.query_impl(collection, bucket, terms, lang, limit, offset)
            }
            (Some(bucket), None, 1) => {
                let bucket = bucket.extract()?;
                let terms = args.get_item(0)?.extract()?;
                self.query_impl(collection, Some(bucket), terms, lang, limit, offset)
            }
            _ => Err(PyOSError::new_err("Invalid arguments"))
        }
    }

    /// Suggest auto-completes words.
    /// The function can be called in two ways:
    ///     `suggest(self, collection, bucket, word, limit=None)`
    ///     `suggest(self, collection, word, limit=None)`
    /// where `bucket` is optional.
    #[pyo3(text_signature = "(self, collection, *args, limit=None, **kwargs)")]
    fn suggest(
        &self,
        collection: &str,
        args: &PyTuple,
        limit: Option<usize>,
        kwargs: Option<&PyDict>,
    ) -> PyResult<Vec<String>> {
        let bucket = kwargs.and_then(|d| d.get_item("bucket"));
        let word = kwargs.and_then(|d| d.get_item("word"));

        match (bucket, word, args.len()) {
            (Some(bucket), Some(word), 0) => {
                let bucket = bucket.extract()?;
                let word = word.extract()?;
                self.suggest_impl(collection, Some(bucket), word, limit)
            }
            (None, Some(word), 0) => {
                let bucket = None;
                let word = word.extract()?;
                self.suggest_impl(collection, bucket, word, limit)
            }
            (None, None, 1) => {
                let bucket = None;
                let word = args.get_item(0)?.extract()?;
                self.suggest_impl(collection, bucket, word, limit)
            }
            (Some(bucket), None, 1) => {
                let bucket = bucket.extract()?;
                let word = args.get_item(0)?.extract()?;
                self.suggest_impl(collection, Some(bucket), word, limit)
            }
            _ => Err(PyOSError::new_err("Invalid arguments"))
        }
    }
}

#[cfg(feature = "ingest")]
#[pyclass(name = "IngestChannel")]
#[pyo3(text_signature = "(self, addr, passwd)")]
struct PyIngestChannel {
    channel: IngestChannel,
}

impl PyIngestChannel {
    /// Push search data in the index.
    fn push_impl(
        &self,
        collection: &str,
        bucket: Option<&str>,
        object: &str,
        text: &str,
        lang: Option<&str>,
    ) -> PyResult<()> {
        let dest = if let Some(bucket) = bucket {
            Dest::col_buc(collection, bucket)
        } else {
            Dest::col(collection)
        };
        let obj_dst = ObjDest::new(dest, object);
        let mut request = PushRequest::new(obj_dst, text);
        if let Some(lang) = lang {
            request.lang = Lang::from_code(lang);
        }
        self.channel
            .push(request)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    /// Pop search data from the index. Returns removed words count as usize type.
    fn pop_impl(
        &self,
        collection: &str,
        bucket: Option<&str>,
        object: &str,
        text: &str,
    ) -> PyResult<usize> {
        let dist = if let Some(bucket) = bucket {
            Dest::col_buc(collection, bucket)
        } else {
            Dest::col(collection)
        };
        let obj_dst = ObjDest::new(dist, object);
        let request = PopRequest::new(obj_dst, text);
        self.channel
            .pop(request)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }
}

#[pymethods]
impl PyIngestChannel {
    #[new]
    fn new(addr: &str, password: &str) -> PyResult<Self> {
        let channel =
            IngestChannel::start(addr, password).map_err(|e| PyOSError::new_err(e.to_string()))?;
        Ok(PyIngestChannel { channel })
    }

    /// Ping server.
    #[pyo3(text_signature = "(self)")]
    fn ping(&self) -> PyResult<()> {
        self.channel
            .ping()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    /// Stop connection.
    #[pyo3(text_signature = "(self)")]
    fn quit(&self) -> PyResult<()> {
        self.channel
            .quit()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    /// Push search data in the index.
    /// The function can be called in two ways:
    ///     `push(self, collection, bucket, object, text, lang=None)`
    ///     `push(self, collection, object, text, lang=None)`
    /// where `bucket` is optional.
    #[pyo3(text_signature = "(self, collection, *args, lang=None, **kwargs)")]
    fn push(
        &self,
        collection: &str,
        args: &PyTuple,
        lang: Option<&str>,
        kwargs: Option<&PyDict>,
    ) -> PyResult<()> {
        let bucket = kwargs.and_then(|d| d.get_item("bucket"));
        let object = kwargs.and_then(|d| d.get_item("object"));
        let text = kwargs.and_then(|d| d.get_item("text"));

        match (bucket, object, text, args.len()) {
            (Some(bucket), Some(object), Some(text), 0) => {
                let bucket = bucket.extract()?;
                let object = object.extract()?;
                let text = text.extract()?;
                self.push_impl(collection, Some(bucket), object, text, lang)
            }
            (None, Some(object), Some(text), 0) => {
                let bucket = None;
                let object = object.extract()?;
                let text = text.extract()?;
                self.push_impl(collection, bucket, object, text, lang)
            }
            (None, None, Some(text), 1) => {
                let bucket = None;
                let object = args.get_item(0)?.extract()?;
                let text = text.extract()?;
                self.push_impl(collection, bucket, object, text, lang)
            }
            (Some(bucket), None, Some(text), 1) => {
                let bucket = bucket.extract()?;
                let object = args.get_item(0)?.extract()?;
                let text = text.extract()?;
                self.push_impl(collection, Some(bucket), object, text, lang)
            }
            _ => Err(PyOSError::new_err("Invalid arguments"))
        }
    }

    /// Pop search data from the index. Returns removed words count as usize type.
    /// The function can be called in two ways:
    ///     `pop(self, collection, bucket, object, text)`
    ///     `pop(self, collection, object, text)`
    /// where `bucket` is optional.
    #[pyo3(text_signature = "(self, collection, *args, **kwargs)")]
    fn pop(
        &self,
        collection: &str,
        args: &PyTuple,
        kwargs: Option<&PyDict>,
    ) -> PyResult<usize> {
        let bucket = kwargs.and_then(|d| d.get_item("bucket"));
        let object = kwargs.and_then(|d| d.get_item("object"));
        let text = kwargs.and_then(|d| d.get_item("text"));

        match (bucket, object, text, args.len()) {
            (Some(bucket), Some(object), Some(text), 0) => {
                let bucket = bucket.extract()?;
                let object = object.extract()?;
                let text = text.extract()?;
                self.pop_impl(collection, Some(bucket), object, text)
            }
            (None, Some(object), Some(text), 0) => {
                let bucket = None;
                let object = object.extract()?;
                let text = text.extract()?;
                self.pop_impl(collection, bucket, object, text)
            }
            (None, None, None, 2) => {
                let bucket = None;
                let object = args.get_item(0)?.extract()?;
                let text = args.get_item(1)?.extract()?;
                self.pop_impl(collection, bucket, object, text)
            }
            (Some(bucket), None, None, 2) => {
                let bucket = bucket.extract()?;
                let object = args.get_item(0)?.extract()?;
                let text = args.get_item(1)?.extract()?;
                self.pop_impl(collection, Some(bucket), object, text)
            }
            _ => Err(PyOSError::new_err("Invalid arguments"))
        }
    }

    /// Count indexed search data of your collection.
    #[pyo3(text_signature = "(self, collection, bucket=None, object=None)")]
    fn count(
        &self,
        collection: &str,
        bucket: Option<&str>,
        object: Option<&str>,
    ) -> PyResult<usize> {
        let request = match (bucket, object) {
            (Some(bucket), Some(object)) => CountRequest::words(collection, bucket, object),
            (Some(bucket), None) => CountRequest::objects(collection, bucket),
            _ => CountRequest::buckets(collection),
        };

        self.channel
            .count(request)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    /// Flush all indexed data from collections.
    #[pyo3(text_signature = "(self, collection, bucket=None, object=None)")]
    fn flush(
        &self,
        collection: &str,
        bucket: Option<&str>,
        object: Option<&str>,
    ) -> PyResult<usize> {
        let request = match (bucket, object) {
            (Some(bucket), Some(object)) => FlushRequest::object(collection, bucket, object),
            (Some(bucket), None) => FlushRequest::bucket(collection, bucket),
            _ => FlushRequest::collection(collection),
        };

        self.channel
            .flush(request)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }
}

#[cfg(feature = "control")]
#[pyclass(name = "ControlChannel")]
#[pyo3(text_signature = "(self, addr, passwd)")]
struct PyControlChannel {
    channel: ControlChannel,
}

#[pymethods]
impl PyControlChannel {
    #[new]
    fn new(addr: &str, password: &str) -> PyResult<Self> {
        let channel =
            ControlChannel::start(addr, password).map_err(|e| PyOSError::new_err(e.to_string()))?;
        Ok(PyControlChannel { channel })
    }

    /// Ping server.
    #[pyo3(text_signature = "(self)")]
    fn ping(&self) -> PyResult<()> {
        self.channel
            .ping()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    /// Stop connection.
    #[pyo3(text_signature = "(self)")]
    fn quit(&self) -> PyResult<()> {
        self.channel
            .quit()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    /// Backup KV + FST to /<BACKUP_{KV/FST}_PATH> See [sonic backend source code](https://github.com/valeriansaliou/sonic/blob/master/src/channel/command.rs#L808) for more information.
    #[pyo3(text_signature = "(self, action)")]
    fn backup(&self, action: &str) -> PyResult<()> {
        self.channel
            .backup(action)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    /// Restore KV + FST from if you already have backup with the same name.
    #[pyo3(text_signature = "(self, action)")]
    fn restore(&self, action: &str) -> PyResult<()> {
        self.channel
            .restore(action)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    /// Consolidate indexed search data instead of waiting for the next automated consolidation tick.
    #[pyo3(text_signature = "(self)")]
    fn consolidate(&self) -> PyResult<()> {
        self.channel
            .consolidate()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }
}

#[pymodule]
fn sonic(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<PySearchChannel>()?;
    #[cfg(feature = "ingest")]
    m.add_class::<PyIngestChannel>()?;
    #[cfg(feature = "control")]
    m.add_class::<PyControlChannel>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4 + 1, 5)
    }
}
