use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use sonic_channel::*;

#[pyclass(name = "SearchChannel")]
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

    fn ping(&self) -> PyResult<bool> {
        self.channel
            .ping()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn quit(&self) -> PyResult<bool> {
        self.channel
            .quit()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn query(&self, collection: &str, bucket: &str, terms: &str) -> PyResult<Vec<String>> {
        self.channel
            .query(collection, bucket, terms)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn query_with_limit(
        &self,
        collection: &str,
        bucket: &str,
        terms: &str,
        limit: usize,
    ) -> PyResult<Vec<String>> {
        self.channel
            .query_with_limit(collection, bucket, terms, limit)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn query_with_limit_and_offset(
        &self,
        collection: &str,
        bucket: &str,
        terms: &str,
        limit: usize,
        offset: usize,
    ) -> PyResult<Vec<String>> {
        self.channel
            .query_with_limit_and_offset(collection, bucket, terms, limit, offset)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn suggest(&self, collection: &str, bucket: &str, terms: &str) -> PyResult<Vec<String>> {
        self.channel
            .suggest(collection, bucket, terms)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn suggest_with_limit(
        &self,
        collection: &str,
        bucket: &str,
        terms: &str,
        limit: usize,
    ) -> PyResult<Vec<String>> {
        self.channel
            .suggest_with_limit(collection, bucket, terms, limit)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }
}

#[cfg(feature = "ingest")]
#[pyclass(name = "IngestChannel")]
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

    fn ping(&self) -> PyResult<bool> {
        self.channel
            .ping()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn quit(&self) -> PyResult<bool> {
        self.channel
            .quit()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn push(&self, collection: &str, bucket: &str, object: &str, text: &str) -> PyResult<bool> {
        self.channel
            .push(collection, bucket, object, text)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn push_with_locale(
        &self,
        collection: &str,
        bucket: &str,
        object: &str,
        text: &str,
        local: &str,
    ) -> PyResult<bool> {
        self.channel
            .push_with_locale(collection, bucket, object, text, local)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn pop(&self, collection: &str, bucket: &str, object: &str, text: &str) -> PyResult<usize> {
        self.channel
            .pop(collection, bucket, object, text)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn bucket_count(&self, collection: &str) -> PyResult<usize> {
        self.channel
            .bucket_count(collection)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn object_count(&self, collection: &str, bucket: &str) -> PyResult<usize> {
        self.channel
            .object_count(collection, bucket)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn word_count(&self, collection: &str, bucket: &str, object: &str) -> PyResult<usize> {
        self.channel
            .word_count(collection, bucket, object)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn flushc(&self, collection: &str) -> PyResult<usize> {
        self.channel
            .flushc(collection)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn flushb(&self, collection: &str, bucket: &str) -> PyResult<usize> {
        self.channel
            .flushb(collection, bucket)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn flusho(&self, collection: &str, bucket: &str, object: &str) -> PyResult<usize> {
        self.channel
            .flusho(collection, bucket, object)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }
}

#[cfg(feature = "control")]
#[pyclass(name = "ControlChannel")]
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

    fn ping(&self) -> PyResult<bool> {
        self.channel
            .ping()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn quit(&self) -> PyResult<bool> {
        self.channel
            .quit()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn backup(&self, action: &str) -> PyResult<bool> {
        self.channel
            .backup(action)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn restore(&self, action: &str) -> PyResult<bool> {
        self.channel
            .restore(action)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    fn consolidate(&self) -> PyResult<bool> {
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
