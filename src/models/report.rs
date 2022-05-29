use super::*;

#[derive(FromRow, Serialize, Debug, Clone)]
pub struct Report {
	pub id: u64,
	#[serde(skip)]
	pub ignored: u8,
	#[serde(serialize_with = "crate::utils::serialize_naive_option")]
	pub solved_at: Option<NaiveDateTime>,
	pub solved_by: Option<u64>,
	pub owner: Option<u64>,
    pub ip: u64,
	pub lang: String,
	#[serde(skip)]
	pub board_id: u64,
	pub board_short: String,
	#[serde(skip)]
	pub thread_id: u64,
	pub thread_num: u64,
	pub comment: String,
	#[serde(serialize_with = "crate::utils::serialize_naive")]
	pub at: NaiveDateTime,

	#[field_default]
	pub post_nums: Vec<u64>
}

pub struct IReport<'a> {
	pub ignored: u8,
	pub owner: Option<u64>,
    pub ip: u64,
	pub lang: &'a str,
	pub board_id: u64,
	pub board_short: &'a str,
	pub thread_id: u64,
	pub thread_num: u64,
	pub comment: &'a str
}

impl Report {
	pub fn is_solved(&self) -> bool {
		self.solved_by.is_some()
	}

	pub async fn get_raw_nums_for_reports(reports: &[u64], conn: Conn) -> Result<(Conn, Vec<(u64, u64)>), MysqlError> {
		let first = match reports.get(0) {
            Some(v) => v,

            None => return Ok((conn, Vec::new())),
        };

        // calculate first number digits count using logarithm hack
        let digits = (*first as f64).log10() as u64;

        // prepare query string
        let mut digits_str = String::with_capacity((digits as usize + 1) * reports.len());

        // bind values
        digits_str.push_str(&first.to_string());
        digits_str.push(',');
        for id in reports {
            digits_str.push_str(&id.to_string());
            digits_str.push(',');
        }

        digits_str.pop();

        let query_str = [
            "SELECT `report_id`, `post_num` FROM `report_post_nums` WHERE `report_id` IN (",
            &digits_str,
            ") ORDER BY `report_id` DESC",
        ]
        .concat();

        conn.prep_exec(
				query_str,
				()
			)
			.await?
			.map_and_drop(|row| (row.get(0).unwrap(), row.get(1).unwrap()))
			.await
	}

	pub async fn find_on_board_id(id: u64, board_id: u64, conn: Conn) -> Result<(Conn, Option<Self>), MysqlError> {
		conn.first_exec(
			"SELECT * FROM `reports` WHERE `id`=:id AND `board_id`=:board_id LIMIT 1",
			params! {
				"id" => id,
				"board_id" => board_id
			}).await
	}

	pub async fn find(id: u64, conn: Conn) -> Result<(Conn, Option<Self>), MysqlError> {
		conn.first_exec(
			"SELECT * FROM `reports` WHERE `id`=:id LIMIT 1",
			params! {
				"id" => id
			}).await
	}

	pub async fn get_page_for_board_id(page: u64, board_id: u64, conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
		conn.prep_exec(
				"SELECT * FROM `reports` WHERE `ignored`=0 AND `board_id`=:board_id ORDER BY `id` DESC LIMIT :limit OFFSET :offset",
				params! { "board_id" => board_id, "limit" => config::REPORTS_PER_PAGE, "offset" => page * config::REPORTS_PER_PAGE }
			)
			.await?
			.map_and_drop(FromRow::from_row)
			.await
	}

	pub async fn get_page_unsolved_for_board_id(page: u64, board_id: u64, conn: Conn) -> Result<(Conn, Vec<Self>), MysqlError> {
		conn.prep_exec(
				"SELECT * FROM `reports` WHERE `ignored`=0 AND `board_id`=:board_id AND `solved_by` IS NULL ORDER BY `id` DESC LIMIT :limit OFFSET :page",
				params! { "board_id" => board_id, "limit" => config::REPORTS_PER_PAGE, "page" => page }
			)
			.await?
			.map_and_drop(FromRow::from_row)
			.await
	}

	pub async fn solve(&mut self, by: u64, conn: Conn) -> Result<Conn, MysqlError> {
		self.solved_at = Some(crate::time_now());
		self.solved_by = Some(by);
		conn.drop_exec(
			"UPDATE `reports` SET `solved_at`=CURRENT_TIMESTAMP, `solved_by`=:by WHERE `id`=:id",
			params! {
				"id" => self.id,
				"by" => by
			}).await
	}

	pub async fn load_posts<'a>(&'a mut self, conn: Conn) -> Result<(Conn, &'a Vec<u64>), MysqlError> {
		let (conn, posts): (Conn, Vec<u64>) = 
			conn.prep_exec(
				"SELECT `post_num` FROM `report_post_nums` WHERE `report_id`=:report_id",
				params! { "report_id" => self.id }
			)
			.await?
			.map_and_drop(|row| {
				row.get(0).unwrap()
			})
			.await?;

		self.post_nums = posts;
		Ok((conn, &self.post_nums))
	}

	// FIXME: transcation support
	pub async fn add_post(&self, post_num: u64, conn: Conn) -> Result<Conn, MysqlError> {
		conn.drop_exec(
			"INSERT INTO `report_post_nums` (report_id, post_num) VALUES (:report_id, :post_num)",
			params! {
				"report_id" => self.id,
				"post_num" => post_num
			}).await
	}

	pub async fn count_amount_last_hour_by_token_id(token_id: u64, conn: Conn) -> Result<(Conn, u64), MysqlError> {
		conn.first_exec(
			"SELECT COUNT(`id`) FROM `reports` WHERE `owner`=:token_id AND `at` > CURRENT_TIMESTAMP - INTERVAL 1 HOUR",
			params! {
				"token_id" => token_id
			}).await
			.map(|(x, y)| (x, y.unwrap()))
	}

	pub async fn count_amount_last_hour_by_ip(ip_id: u64, conn: Conn) -> Result<(Conn, u64), MysqlError> {
		conn.first_exec(
			"SELECT COUNT(`id`) FROM `reports` WHERE `ip`=:ip_id AND `at` > CURRENT_TIMESTAMP - INTERVAL 1 HOUR",
			params! {
				"ip_id" => ip_id
			}).await
			.map(|(x, y)| (x, y.unwrap()))
	}
}

impl IReport<'_> {
	/* dynamic */
    pub async fn create(&self, conn: Conn) -> Result<(Conn, Report), MysqlError> {
        let conn = conn
            .drop_exec(
                "INSERT INTO `reports` (owner, ignored, ip, lang, board_id, board_short, thread_id, thread_num, comment) VALUES (:owner, :ignored, :ip, :lang, :board_id, :board_short, :thread_id, :thread_num, :comment)",
                params! {
                	"ignored" => self.ignored,
                	"owner" => self.owner,
                	"ip" => self.ip,
                	"lang" => self.lang,
                	"board_id" => self.board_id,
                	"board_short" => self.board_short,
                	"thread_id" => self.thread_id,
                	"thread_num" => self.thread_num,
                	"comment" => self.comment,
                },
            )
            .await?;

        let (conn, row): (_, Option<Row>) = conn
            .first_exec(
                "SELECT * FROM `reports` WHERE ID=(SELECT LAST_INSERT_ID())",
                (),
            )
            .await?;

        Ok((conn, Report::from_row(row.unwrap())))
    }
}