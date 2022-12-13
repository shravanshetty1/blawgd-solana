use anyhow::anyhow;
use anyhow::Result;

pub struct Window {
    window: web_sys::Window,
}

pub fn new_window(window: web_sys::Window) -> Window {
    Window { window }
}

impl Window {
    pub fn inner(&self) -> web_sys::Window {
        self.window.clone()
    }
    pub fn location(&self) -> Location {
        Location {
            location: self.window.location(),
        }
    }

    pub fn document(&self) -> Result<Document> {
        Ok(Document {
            document: self
                .window
                .document()
                .ok_or(anyhow!("could not get document object from window"))?,
        })
    }
}

pub struct Element {
    inner: web_sys::Element,
}

impl Element {
    pub fn inner(&self) -> web_sys::Element {
        self.inner.clone()
    }
    pub fn set_inner_html(&self, val: &str) {
        self.inner.set_inner_html(val)
    }
    pub fn inner_html(&self) -> String {
        self.inner.inner_html()
    }
}

#[derive(Clone)]
pub struct Document {
    document: web_sys::Document,
}

impl Document {
    pub fn inner(&self) -> web_sys::Document {
        self.document.clone()
    }
    
    pub fn get_element_by_id(&self, id: &str) -> Result<Element> {
        Ok(Element {
            inner: self
                .document
                .get_element_by_id(id)
                .ok_or(anyhow!("could not find element with id {}", id))?,
        })
    }
}

pub struct Location {
    location: web_sys::Location,
}

impl Location {
    pub fn inner(&self) -> web_sys::Location {
        self.location.clone()
    }
    pub fn href(&self) -> Result<String> {
        Ok(self
            .location
            .href()
            .map_err(|_| anyhow!("could not get href from location object"))?)
    }
    pub fn protocol(&self) -> Result<String> {
        Ok(self
            .location
            .protocol()
            .map_err(|_| anyhow!("could not get protocol from location object"))?)
    }
    pub fn hostname(&self) -> Result<String> {
        Ok(self
            .location
            .hostname()
            .map_err(|_| anyhow!("could not get hostname from location object"))?)
    }
    pub fn port(&self) -> Result<String> {
        Ok(self
            .location
            .port()
            .map_err(|_| anyhow!("could not get port from location object"))?)
    }
}
