#[cfg(feature = "github-login")]
pub mod github;
#[cfg(feature = "gitlab-login")]
pub mod gitlab;
#[cfg(feature = "google-login")]
pub mod google;
#[cfg(feature = "microsoft-login")]
pub mod microsoft;

