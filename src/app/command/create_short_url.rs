use crate::id_provider::IDProvider;

pub struct CreateShortUrl<I>
where
    I: IDProvider,
{
    id_provider: I,
}

impl<I> CreateShortUrl<I>
where
    I: IDProvider,
{
    pub fn new(id_provider: I) -> Self {
        Self { id_provider }
    }

    pub async fn execute(&self, full_url: String) -> Result<String, String> {
        let _id = self.id_provider.provide();
        Ok(_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::id_provider;

    use super::*;

    #[tokio::test]
    async fn get_short_url() {
        // Given
        let id_provider = id_provider::FakeIDProvider::new("123".to_owned());
        let command = CreateShortUrl::new(id_provider);

        // When
        let result = command.execute("https://google.com".to_owned()).await;

        // Then
        assert_ne!(result, Ok("".to_owned()));
    }

    #[tokio::test]
    async fn get_two_different_short_url() {
        // Given
        let id_provider = id_provider::FakeIDProvider::new("123".to_owned());
        let command = CreateShortUrl::new(id_provider);

        // When
        let result1 = command.execute("https://google.com".to_owned()).await;
        let result2 = command.execute("https://google.com".to_owned()).await;

        // Then
        assert_ne!(result1, result2);
    }
}
