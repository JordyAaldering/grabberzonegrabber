use serde::{Deserialize, Serialize};

use crate::Page;

/// Wrapper to holds all pages of the book.
#[derive(Clone, Default, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct ArrayOfComicPageInfo {
    #[serde(rename = "Page")]
    pub pages: Vec<Page>,
}

impl ArrayOfComicPageInfo {
    pub fn merge(&self, other: &Self) -> Self {
        let mut l = self.pages.clone();
        let mut r = other.pages.clone();
        l.sort();
        r.sort();

        let mut il = 0;
        let mut ir = 0;
        let mut pages = Vec::new();
        while (il < l.len()) && (ir < r.len()) {
            if l[il] < r[ir] {
                pages.push(l[il].clone());
                il += 1;
            } else if l[il] > r[ir] {
                pages.push(r[ir].clone());
                ir += 1;
            } else {
                // Try to merge pages with the same page number
                pages.push(l[il].merge(&r[ir]));
                il += 1;
                ir += 1;
            }
        }

        while il < l.len() {
            pages.push(l[il].clone());
            il += 1;
        }

        while ir < r.len() {
            pages.push(r[ir].clone());
            ir += 1;
        }

        Self { pages }
    }
}
