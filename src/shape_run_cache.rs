#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};
use core::ops::Range;

use crate::{AttrsList, AttrsOwned, HashMap, ShapeGlyph};

/// Key for caching shape runs.
///
/// Attributes are ids from [`ShapeRunCache`]'s interner: two ids are equal
/// exactly when the [`AttrsOwned`] values they stand for are equal.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ShapeRunKey {
    pub text: String,
    pub default_attrs: usize,
    pub attrs_spans: Vec<(Range<usize>, usize)>,
}

/// A helper structure for caching shape runs.
#[derive(Clone, Default)]
#[cfg_attr(not(feature = "shape-run-cache"), allow(dead_code))]
pub struct ShapeRunCache {
    age: u64,
    cache: HashMap<ShapeRunKey, (u64, Vec<ShapeGlyph>)>,
    /// Allocation reuse for lookup keys that hit the cache.
    pub(crate) scratch_key: Option<ShapeRunKey>,
    /// Every distinct attribute set seen, with its id.
    attrs_ids: HashMap<AttrsOwned, usize>,
    /// Version of the attribute list last seen, with the id of its defaults.
    list: Option<(u64, usize)>,
    /// Ids of that list's spans, by span start.
    span_ids: HashMap<usize, usize>,
}

#[cfg_attr(not(feature = "shape-run-cache"), allow(dead_code))]
impl ShapeRunCache {
    fn intern(attrs_ids: &mut HashMap<AttrsOwned, usize>, attrs: &AttrsOwned) -> usize {
        if let Some(&id) = attrs_ids.get(attrs) {
            return id;
        }
        let id = attrs_ids.len();
        attrs_ids.insert(attrs.clone(), id);
        id
    }

    /// Id of `attrs_list`'s defaults. Selects `attrs_list` for [`Self::span_attrs_id`].
    pub(crate) fn default_attrs_id(&mut self, attrs_list: &AttrsList) -> usize {
        match self.list {
            Some((version, id)) if version == attrs_list.version() => id,
            _ => {
                self.span_ids.clear();
                let id = Self::intern(&mut self.attrs_ids, attrs_list.defaults_owned());
                self.list = Some((attrs_list.version(), id));
                id
            }
        }
    }

    /// Id of the span starting at `start` in the list last passed to
    /// [`Self::default_attrs_id`], whose attributes are `attrs`.
    pub(crate) fn span_attrs_id(&mut self, start: usize, attrs: &AttrsOwned) -> usize {
        if let Some(&id) = self.span_ids.get(&start) {
            return id;
        }
        let id = Self::intern(&mut self.attrs_ids, attrs);
        self.span_ids.insert(start, id);
        id
    }

    /// Get cache item, updating age if found
    pub fn get(&mut self, key: &ShapeRunKey) -> Option<&Vec<ShapeGlyph>> {
        self.cache.get_mut(key).map(|(age, glyphs)| {
            *age = self.age;
            &*glyphs
        })
    }

    /// Insert cache item with current age
    pub fn insert(&mut self, key: ShapeRunKey, glyphs: Vec<ShapeGlyph>) {
        self.cache.insert(key, (self.age, glyphs));
    }

    /// Remove anything in the cache with an age older than `keep_ages`
    pub fn trim(&mut self, keep_ages: u64) {
        self.cache
            .retain(|_key, (age, _glyphs)| *age + keep_ages >= self.age);
        // Increase age
        self.age += 1;
    }
}

impl core::fmt::Debug for ShapeRunCache {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ShapeRunCache").finish()
    }
}
