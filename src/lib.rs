use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use sonic_channel::*;

#[pyclass(name = "SearchChannel")]
#[pyo3(text_signature = "(self, addr, passwd)")]
struct PySearchChannel {
    channel: SearchChannel,
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
    #[pyo3(text_signature = "(self, collection, bucket, terms, lang, limit, offset)")]
    fn query(
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
    #[pyo3(text_signature = "(self, collection, bucket, word, limit)")]
    fn suggest(
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

#[cfg(feature = "ingest")]
#[pyclass(name = "IngestChannel")]
#[pyo3(text_signature = "(self, addr, passwd)")]
struct PyIngestChannel {
    channel: IngestChannel,
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
    #[pyo3(text_signature = "(self, collection, bucket, object, text, lang)")]
    fn push(
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
    #[pyo3(text_signature = "(self, collection, bucket, object, text)")]
    fn pop(
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

    /// Count indexed search data of your collection.
    #[pyo3(text_signature = "(self, collection, bucket, object)")]
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
    #[pyo3(text_signature = "(self, collection, bucket, object)")]
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
