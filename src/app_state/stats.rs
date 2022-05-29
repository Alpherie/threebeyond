use super::*;

#[derive(Clone, Serialize, threebeyond_codegen_from_row::FromRow)]
pub struct StatisticsCache {
    pub total_domains: u64,
    pub total_boards: u64,
    pub total_mods: u64,
    pub total_donated: u64,
    pub total_threads: u64,
    pub total_posts: u64,
    pub total_posts_last_hour: u64,
    pub total_posts_last_day: u64,
    pub total_posts_last_week: u64,
    pub total_posts_last_month: u64,
    pub total_posts_last_year: u64,
    pub total_media_files: u64,
    pub total_media_size: u64,

    #[field_default]
    pub donations: Vec<models::Donation>,
}

impl StatisticsCache {
    pub async fn get(conn: Conn) -> Result<(Conn, Self), MysqlError> {
        // IDEA: inner joins...
        let (conn, mut stats) = conn.first_exec("
			SELECT  (SELECT count(`langs`.`code`) FROM `langs`) AS `total_domains`,
					(SELECT count(`boards`.`id`) FROM `boards`) AS `total_boards`,
					(SELECT count(`tokens`.`id`) FROM `tokens` WHERE `tokens`.`is_mod`=1) AS `total_mods`,
					(SELECT COALESCE(sum(`donations`.`amount_usd`),0) FROM `donations`) AS `total_donated`,
					(SELECT count(`threads`.`id`) FROM `threads`) AS `total_threads`,
					(SELECT count(`posts`.`id`) FROM `posts`) AS `total_posts`,
					(SELECT count(`posts`.`id`) FROM `posts` WHERE `posts`.`at` >= NOW() - INTERVAL 1 HOUR) AS `total_posts_last_hour`,
					(SELECT count(`posts`.`id`) FROM `posts` WHERE `posts`.`at` >= NOW() - INTERVAL 1 DAY) AS `total_posts_last_day`,
					(SELECT count(`posts`.`id`) FROM `posts` WHERE `posts`.`at` >= NOW() - INTERVAL 1 WEEK) AS `total_posts_last_week`,
					(SELECT count(`posts`.`id`) FROM `posts` WHERE `posts`.`at` >= NOW() - INTERVAL 1 MONTH) AS `total_posts_last_month`,
					(SELECT count(`posts`.`id`) FROM `posts` WHERE `posts`.`at` >= NOW() - INTERVAL 1 YEAR) AS `total_posts_last_year`,
					(SELECT count(`files`.`uuid`) FROM `files`) AS `total_media_files`,
					(SELECT COALESCE(sum(`files`.`size`),0) FROM `files`) AS `total_media_size`
		", ())
			.await
			.map(|(c, v)| (c, Self::from_row(v.unwrap())))?;

        let (conn, donations) = models::Donation::all(conn).await?;
        stats.donations = donations;
        Ok((conn, stats))
    }
}
