use beyond;

-- INSERT INTO `langs` (code, name) VALUES ("en", "English");
-- INSERT INTO `langs` (code, name) VALUES ("ru", "Русский");
-- INSERT INTO `langs` (code, name) VALUES ("jp", "日本語");

-- INSERT INTO `boards` (short, name, description, pages_count, per_page, last_replies)
-- VALUES ("dg", "General discussion", "About service...", 10, 10, 5);

-- perms
-- INSERT INTO `ban_reasons` (alias) VALUES ("CP");
-- INSERT INTO `perms` (id, name) VALUES (1, "langs::en::boards::b::posts::delete");
-- INSERT INTO `perms` (id, name) VALUES (2, "langs::en::boards::b::posts::ban");
-- INSERT INTO `perms` (id, name) VALUES (3, "langs::en::boards::b::posts::ban_subnet");
-- INSERT INTO `perms` (id, name) VALUES (4, "langs::en::boards::b::reports::list");
-- INSERT INTO `perms` (id, name) VALUES (5, "langs::en::boards::b::reports::solve");
-- INSERT INTO `roles` (id, name) VALUES (1, "langs::en::boards::b::moderator");
-- INSERT INTO `role_perms` (role, perm) VALUES (1, 1);
-- INSERT INTO `role_perms` (role, perm) VALUES (1, 2);
-- INSERT INTO `role_perms` (role, perm) VALUES (1, 3);
-- INSERT INTO `role_perms` (role, perm) VALUES (1, 4);
-- INSERT INTO `role_perms` (role, perm) VALUES (1, 5);
-- INSERT INTO `perms` (id, name) VALUES (6, "reasons::create");
-- INSERT INTO `roles` (id, name) VALUES (2, "root");
-- INSERT INTO `role_perms` (role, perm) VALUES (2, 6);