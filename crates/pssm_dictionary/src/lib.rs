mod dict;

use std::hash::Hash;
use pssm_core::{transition::{IntoTransitionMut, TransitionMut}, StateMachine};

pub use dict::Dictionary;

/// A dictionary of transitions.
/// 
/// This dictionary is a collection of transitions that can be run in a state machine. This dictionary
/// can also contain folders of transitions, which are dictionaries of transitions that can be run in
/// a state machine.
/// 
/// # Examples
/// 
/// ```
/// use pssm_core::{StateMachine, Truth};
/// use pssm_macro::*;
/// use pssm_dictionary::TransitionDictionary;
/// 
/// #[derive(Debug,Truth)]
/// struct A();
/// 
/// fn insert_a() -> A {
///     A()
/// }
/// 
/// let mut state_machine = StateMachine::new();
/// 
/// let mut transitions = TransitionDictionary::new();
/// transitions.add_transition("insert_a", insert_a).unwrap();
/// 
/// transitions
///     .runnable_transitions(&state_machine)
///     .run(&"insert_a", &mut state_machine);
/// 
/// assert!(state_machine.has_truth::<A>());
/// ```
pub type TransitionDictionary<'a,K> = Dictionary<K, TransitionMut<'a>>;

impl<'a,K: Hash + Eq + Clone> TransitionDictionary<'a,K> {
    /// Returns a dictionary to all transitions that can be run in the given state.
    /// 
    /// This function will return a dictionary of references to all transitions in this dictionary
    /// that can be run in the given state. This function will also recursively check all folders
    /// in this dictionary for transitions that can be run in the given state.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm_core::{StateMachine, Truth};
    /// use pssm_macro::Truth;
    /// use pssm_dictionary::TransitionDictionary;
    /// 
    /// #[derive(Debug,Truth)]
    /// struct A();
    /// 
    /// fn insert_a() -> A {
    ///         A()
    /// }
    /// 
    /// fn use_a(a: A) {
    ///     println!("{:?}", a);
    /// }
    /// 
    /// let mut state_machine = StateMachine::new();
    /// 
    /// let mut transitions = TransitionDictionary::new();
    /// transitions.add_transition("insert_a", insert_a).unwrap();
    /// transitions.add_transition("use_a", use_a).unwrap();
    /// 
    /// let runnables = transitions.runnable_transitions(&state_machine);
    ///     
    /// assert!(runnables.has(&"insert_a"));
    /// assert!(!runnables.has(&"use_a"));
    /// ```
    pub fn runnable_transitions(&mut self, state: &StateMachine) -> Dictionary<K, &mut TransitionMut<'a>> {
        let mut runnables = Dictionary::new();

        for (key, transition) in &mut self.entries {
            if state.can_run_transition_mut(transition) {
                runnables.insert(key.clone(), transition);
            }
        }

        for (key, folder) in &mut self.folders {
            let folder_runnables = folder.runnable_transitions(state);

            if folder_runnables.no_values() && folder_runnables.no_folders() {
                continue;
            }

            runnables.insert_folder(key.clone(), folder_runnables);
        }

        runnables
    }

    /// Adds a transition to this dictionary.
    /// 
    /// This function will add a transition to this dictionary with the given key. If a transition
    /// with the given key already exists in this dictionary, it will be replaced with the new
    /// transition.
    /// Returns the old transition if it exists.
    /// Returns an error if the transition is not valid.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm_core::{StateMachine, Truth};
    /// use pssm_macro::Truth;
    /// use pssm_dictionary::TransitionDictionary;
    /// 
    /// #[derive(Debug,Truth)]
    /// struct A();
    /// 
    /// fn insert_a() -> A {
    ///     A()
    /// }
    /// 
    /// let mut transitions = TransitionDictionary::new();
    /// transitions.add_transition("insert_a", insert_a).unwrap();
    /// 
    /// assert!(transitions.has(&"insert_a"));
    /// ```
    pub fn add_transition<T,In,Marker>(&mut self, key: K, transition: T) -> Result<Option<TransitionMut>,&'static str>
    where 
        T: IntoTransitionMut<'a,In,Marker>
    {
        let transition = transition.into_transition_mut()?;
        Ok(self.insert(key, transition))
    }
}

#[macro_export]
macro_rules! transition_dictionary {
    ($($key:ident = $val:expr),* ; $($folder:ident { $($contents:tt)* }),* $(,)? ) => {{
        let mut dict = $crate::TransitionDictionary::new();
        $(
            dict.add_transition(stringify!($key).into(), $val).unwrap();
        )*
        $(
            let mut folder = transition_dictionary!($($contents)*);
            dict.insert_folder(stringify!($folder).into(), folder);
        )*
        dict
    }};
    ($($key:ident = $val:expr),* $(,)? ) => {{
        let mut dict = $crate::TransitionDictionary::new();
        $(
            dict.add_transition(stringify!($key).into(), $val).unwrap();
        )*
        dict
    }};
    ($($folder:ident { $($contents:tt)* }),* $(,)? ) => {{
        let mut dict = $crate::TransitionDictionary::new();
        $(
            let mut folder = transition_dictionary!($($contents)*);
            dict.insert_folder(stringify!($folder).into(), folder);
        )*
        dict
    }};
}