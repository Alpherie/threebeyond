use super::*;

/* TO DO */
#[derive(FromRow, Serialize, Debug, Clone)]
pub struct Donation {
    pub id: u64,
    pub name: String,
    pub currency: String,
    pub amount: u64,
    pub amount_usd: u64,
    pub at: NaiveDateTime,
}

pub struct IDonation<'a> {
    pub name: &'a str,
    pub currency: &'a str,
    pub amount: u64,
    pub amount_usd: u64,
}

impl IDonation<'_> {
    /* dynamic */
    pub async fn create(&self, conn: Conn) -> Result<(Conn, Donation), MysqlError> {
        let conn = conn
            .drop_exec(
                "INSERT INTO `donation` (name, currency, amount, amount_usd) VALUES (:name, :currency, :amount, :amount_usd)",
                params! {
                    "name" => self.name,
                    "currency" => self.currency,
                    "amount" => self.amount,
                    "amount_usd" => self.amount_usd
                },
            )
            .await?;

        let (conn, row): (_, Option<Row>) = conn
            .first_exec(
                "SELECT * FROM `donations` WHERE ID=(SELECT LAST_INSERT_ID())",
                (),
            )
            .await?;

        Ok((conn, Donation::from_row(row.unwrap())))
    }
}

impl Donation {
    pub async fn all(conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
        conn.prep_exec("SELECT * FROM `donations`", ())
            .await?
            .map_and_drop(Self::from_row)
            .await
    }
}
