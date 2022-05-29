use super::*;

#[derive(Deserialize)]
pub struct DonationCreate {
    pub name: String,
    pub currency: String,
    pub amount: u64,
    pub amount_usd: u64,
}

#[post("")]
pub async fn create(
    pool: Data<Pool>,
    mut auth: models::Token,
    donation: Json<DonationCreate>,
) -> Result<impl Responder, ApiError> {
    let conn = auth_gen::perm!(auth, "donations::create", pool.get_conn().await?);
    let (_, conn) = models::IDonation {
        name: &donation.name,
        currency: &donation.currency,
        amount: donation.amount,
        amount_usd: donation.amount_usd,
    }
    .create(conn)
    .await?;

    res::no!()
}
