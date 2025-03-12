use std::{collections::HashMap, hash::Hash};

/// A dictionary that can store values and other dictionaries.
///
/// This dictionary is designed to be used in a hierarchical manner, where each key can be a value or another dictionary.
/// This allows for a tree-like structure to be created, where values can be stored at any level of the tree.
/// 
/// # Examples
/// 
/// ```
/// use sems_dictionary::Dictionary;
/// 
/// let mut dict = Dictionary::new();
/// 
/// dict.insert("key", 5);
/// 
/// let mut folder = Dictionary::new();
/// folder.insert("key", 10);
/// 
/// dict.insert_folder("folder", folder);
/// 
/// assert_eq!(dict.get(&"key"), Some(&5));
/// assert_eq!(dict.get_deep(&["folder", "key"]), Some(&10));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dictionary<K: Hash + Eq, V> {
    pub(crate) entries: HashMap<K, V>,
    pub(crate) folders: HashMap<K, Dictionary<K,V>>
}

impl<K: Hash + Eq, V> Dictionary<K,V> {
    /// Creates a new, empty dictionary.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let dict = Dictionary::<&str, i32>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            folders: HashMap::new()
        }
    }

    /// Inserts a value into the dictionary.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// 
    /// dict.insert("key", 5);
    /// assert_eq!(dict.get(&"key"), Some(&5));
    /// 
    /// let old_value = dict.insert("key", 10);
    /// assert_eq!(old_value, Some(5));
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.entries.insert(key, value)
    }

    /// Inserts a sub-dictionary into the dictionary.
    /// 
    /// # Examples
    ///
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// 
    /// let mut folder = Dictionary::new();
    /// folder.insert("key", 10);
    /// 
    /// dict.insert_folder("folder", folder);
    /// assert_eq!(dict.get_deep(&["folder", "key"]), Some(&10));
    /// 
    /// let old_folder = dict.insert_folder("folder", Dictionary::new());
    /// assert!(old_folder.is_some());
    /// ```
    pub fn insert_folder(&mut self, key: K, folder: Dictionary<K,V>) -> Option<Dictionary<K,V>> {
        self.folders.insert(key, folder)
    }

    /// Returns a reference to the value associated with the given key.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// dict.insert("key", 5);
    /// 
    /// assert_eq!(dict.get(&"key"), Some(&5));
    /// assert_eq!(dict.get(&"missing"), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.get(key)
    }

    /// Returns a mutable reference to the value associated with the given key.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// dict.insert("key", 5);
    /// 
    /// if let Some(value) = dict.get_mut(&"key") {
    ///    *value = 10;
    /// }
    /// 
    /// assert_eq!(dict.get(&"key"), Some(&10));
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.entries.get_mut(key)
    }

    /// Returns a reference to the sub-dictionary associated with the given key.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::<&str, i32>::new();
    /// 
    /// dict.insert_folder("folder", Dictionary::new());
    /// 
    /// assert!(dict.get_folder(&"folder").is_some());
    /// assert!(dict.get_folder(&"missing").is_none());
    /// ```
    pub fn get_folder(&self, key: &K) -> Option<&Dictionary<K,V>> {
        self.folders.get(key)
    }

    /// Returns a mutable reference to the sub-dictionary associated with the given key.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// 
    /// dict.insert_folder("folder", Dictionary::new());
    /// 
    /// if let Some(folder) = dict.get_folder_mut(&"folder") {
    ///    folder.insert("key", 5);
    /// }
    /// 
    /// assert_eq!(dict.get_deep(&["folder", "key"]), Some(&5));
    /// ```
    pub fn get_folder_mut(&mut self, key: &K) -> Option<&mut Dictionary<K,V>> {
        self.folders.get_mut(key)
    }

    /// Returns a reference to the value associated with the given key path.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// dict.insert("key", 5);
    /// 
    /// let mut folder = Dictionary::new();
    /// folder.insert("key", 10);
    /// 
    /// dict.insert_folder("folder", folder);
    /// 
    /// assert_eq!(dict.get_deep(&["key"]), Some(&5));
    /// assert_eq!(dict.get_deep(&["folder", "key"]), Some(&10));
    /// ```
    pub fn get_deep(&self, keys: &[K]) -> Option<&V> {
        if keys.len() == 1 {
            self.get(&keys[0])
        } else {
            if let Some(folder) = self.folders.get(&keys[0]) {
                folder.get_deep(&keys[1..])
            } else {
                None
            }
        }
    }

    /// Returns a mutable reference to the value associated with the given key path.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// dict.insert("key", 5);
    /// 
    /// let mut folder = Dictionary::new();
    /// folder.insert("key", 10);
    /// 
    /// dict.insert_folder("folder", folder);
    /// 
    /// if let Some(value) = dict.get_deep_mut(&["folder", "key"]) {
    ///     *value = 15;
    /// }
    /// 
    /// assert_eq!(dict.get_deep(&["folder", "key"]), Some(&15));
    /// ```
    pub fn get_deep_mut(&mut self, keys: &[K]) -> Option<&mut V> {
        if keys.len() == 1 {
            self.get_mut(&keys[0])
        } else {
            if let Some(folder) = self.folders.get_mut(&keys[0]) {
                folder.get_deep_mut(&keys[1..])
            } else {
                None
            }
        }
    }

    /// Removes the value associated with the given key.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// dict.insert("key", 5);
    /// 
    /// assert_eq!(dict.remove(&"key"), Some(5));
    /// assert_eq!(dict.remove(&"missing"), None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.entries.remove(key)
    }

    /// Removes the sub-dictionary associated with the given key.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// 
    /// let mut folder = Dictionary::new();
    /// folder.insert("key", 10);
    /// 
    /// dict.insert_folder("folder", folder);
    /// 
    /// assert!(dict.remove_folder(&"folder").is_some());
    /// assert!(dict.remove_folder(&"missing").is_none());
    /// ```
    pub fn remove_folder(&mut self, key: &K) -> Option<Dictionary<K,V>> {
        self.folders.remove(key)
    }

    /// Removes the value associated with the given key path.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// dict.insert("key", 5);
    /// 
    /// let mut folder = Dictionary::new();
    /// folder.insert("key", 10);
    /// 
    /// dict.insert_folder("folder", folder);
    /// 
    /// assert_eq!(dict.remove_deep(&["key"]), Some(5));
    /// assert_eq!(dict.remove_deep(&["folder", "key"]), Some(10));
    /// ```
    pub fn remove_deep(&mut self, keys: &[K]) -> Option<V> {
        if keys.len() == 1 {
            self.remove(&keys[0])
        } else {
            if let Some(folder) = self.folders.get_mut(&keys[0]) {
                folder.remove_deep(&keys[1..])
            } else {
                None
            }
        }
    }

    /// Checks if the dictionary contains a value associated with the given key.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// dict.insert("key", 5);
    /// 
    /// assert!(dict.has(&"key"));
    /// assert!(!dict.has(&"missing"));
    /// ```
    pub fn has(&self, key: &K) -> bool {
        self.entries.contains_key(key)
    }

    /// Checks if the dictionary contains a sub-dictionary associated with the given key.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::<&str, i32>::new();
    /// 
    /// let mut folder = Dictionary::new();
    /// 
    /// dict.insert_folder("folder", folder);
    /// 
    /// assert!(dict.has_folder(&"folder"));
    /// assert!(!dict.has_folder(&"missing"));
    /// ```
    pub fn has_folder(&self, key: &K) -> bool {
        self.folders.contains_key(key)
    }

    /// Checks if the dictionary contains a value associated with the given key path.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// dict.insert("key", 5);
    /// 
    /// let mut folder = Dictionary::new();
    /// folder.insert("key", 10);
    /// 
    /// dict.insert_folder("folder", folder);
    /// 
    /// assert!(dict.has_deep(&["key"]));
    /// assert!(dict.has_deep(&["folder", "key"]));
    /// assert!(!dict.has_deep(&["missing"]));
    /// ```
    pub fn has_deep(&self, keys: &[K]) -> bool {
        if keys.len() == 1 {
            self.has(&keys[0])
        } else {
            if let Some(folder) = self.folders.get(&keys[0]) {
                folder.has_deep(&keys[1..])
            } else {
                false
            }
        }
    }

    /// Returns the number of values stored in the dictionary.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// 
    /// dict.insert("key1", 5);
    /// dict.insert("key2", 10);
    /// 
    /// assert_eq!(dict.value_count(), 2);
    /// ```
    pub fn value_count(&self) -> usize {
        self.entries.len()
    }
    
    /// Returns the number of sub-dictionaries stored in the dictionary.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::<&str, i32>::new();
    /// 
    /// let mut folder1 = Dictionary::new();
    /// let mut folder2 = Dictionary::new();
    /// 
    /// dict.insert_folder("folder1", folder1);
    /// dict.insert_folder("folder2", folder2);
    /// 
    /// assert_eq!(dict.folder_count(), 2);
    /// ```
    pub fn folder_count(&self) -> usize {
        self.folders.len()
    }

    /// Checks if the dictionary has no values stored in it.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::<&str, i32>::new();
    /// 
    /// assert!(dict.no_values());
    /// ```
    pub fn no_values(&self) -> bool {
        self.entries.is_empty()
    }

    /// Checks if the dictionary has no sub-dictionaries stored in it.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::<&str, i32>::new();
    /// 
    /// assert!(dict.no_folders());
    /// ```
    pub fn no_folders(&self) -> bool {
        self.folders.is_empty()
    }

    /// Returns an iterator over the values stored in the dictionary.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// 
    /// dict.insert("key1", 5);
    /// dict.insert("key2", 10);
    /// 
    /// for (key, value) in dict.iter() {
    ///    println!("{}: {}", key, value);
    /// }
    pub fn iter(&self) -> impl Iterator<Item=(&K,&V)> {
        self.entries.iter()
    }

    /// Returns an iterator over the values stored in the dictionary.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// 
    /// dict.insert("key1", 5);
    /// dict.insert("key2", 10);
    /// 
    /// for (key, value) in dict.iter_mut() {
    ///     *value += 1;
    /// }
    /// 
    /// assert_eq!(dict.get(&"key1"), Some(&6));
    /// assert_eq!(dict.get(&"key2"), Some(&11));
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item=(&K,&mut V)> {
        self.entries.iter_mut()
    }
    
    /// Returns an iterator over the sub-dictionaries stored in the dictionary.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::<&str, i32>::new();
    /// 
    /// let mut folder1 = Dictionary::new();
    /// let mut folder2 = Dictionary::new();
    /// 
    /// dict.insert_folder("folder1", folder1);
    /// dict.insert_folder("folder2", folder2);
    /// 
    /// for (key, folder) in dict.iter_folders() {
    ///     println!("{}: {:?}", key, folder);
    /// }
    /// ```
    pub fn iter_folders(&self) -> impl Iterator<Item=(&K,&Dictionary<K,V>)> {
        self.folders.iter()
    }

    /// Returns an iterator over the sub-dictionaries stored in the dictionary.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_dictionary::Dictionary;
    /// 
    /// let mut dict = Dictionary::new();
    /// 
    /// let mut folder1 = Dictionary::new();
    /// let mut folder2 = Dictionary::new();
    /// 
    /// dict.insert_folder("folder1", folder1);
    /// dict.insert_folder("folder2", folder2);
    /// 
    /// for (key, folder) in dict.iter_folders_mut() {
    ///    folder.insert("key", 5);
    /// }
    /// 
    /// assert_eq!(dict.get_deep(&["folder1", "key"]), Some(&5));
    /// assert_eq!(dict.get_deep(&["folder2", "key"]), Some(&5));
    /// ```
    pub fn iter_folders_mut(&mut self) -> impl Iterator<Item=(&K,&mut Dictionary<K,V>)> {
        self.folders.iter_mut()
    }
}