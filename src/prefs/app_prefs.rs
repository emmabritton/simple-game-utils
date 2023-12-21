use crate::prefs::*;
use log::error;
use serde::Serialize;

const FILE_NAME: &str = "app.prefs";
const KEY: &str = "settings";

///
/// # Usage
///
/// ```
///# use serde::{Deserialize, Serialize};
///# use simple_game_utils::prefs::app_prefs::AppPrefs;
///# #[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// struct Settings {
///     user: String,
///     theme: usize
/// }
///
/// let mut prefs: AppPrefs<Settings> = AppPrefs::new("com","example","readme", || Settings::default()).unwrap();
/// println!("{}", prefs.data.user);
/// prefs.data.user = String::from("New");
/// prefs.save();
///```
///
pub struct AppPrefs<T: Sized + Clone + Serialize + DeserializeOwned> {
    prefs: Preferences<T>,
    pub data: T,
}

impl<T: Sized + Clone + Serialize + DeserializeOwned> AppPrefs<T> {
    pub fn new(
        qualifier: &str,
        organization: &str,
        application: &str,
        default: fn() -> T,
    ) -> Result<Self, GameUtilError> {
        let mut prefs: Preferences<T> = Preferences::new(
            get_pref_dir(qualifier, organization, application)?,
            FILE_NAME,
        );
        if let Err(e) = prefs.load() {
            error!("Unable to restore app prefs: {e:?}");
        }
        let data = if let Some(data) = prefs.get(KEY) {
            data.clone()
        } else {
            default()
        };
        Ok(AppPrefs { prefs, data })
    }
}

impl<T: Sized + Clone + Serialize + DeserializeOwned> AppPrefs<T> {
    pub fn save(&mut self) {
        self.prefs.set(KEY, self.data.clone());
        if let Err(e) = self.prefs.save() {
            error!("Unable to save app prefs: {e:?}");
        }
    }
}
