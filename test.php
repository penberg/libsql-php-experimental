<?php

$db = new SQLite3('my_database.db');

$db->exec('CREATE TABLE IF NOT EXISTS friends (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)');
$db->exec("INSERT INTO friends (name, age) VALUES ('Alice', 30)");
$db->exec("INSERT INTO friends (name, age) VALUES ('Bob', 35)");

