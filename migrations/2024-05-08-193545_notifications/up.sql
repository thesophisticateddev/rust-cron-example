-- Your SQL goes here
CREATE TABLE "notifications"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"title" VARCHAR NOT NULL,
	"body" TEXT NOT NULL,
	"published" BOOL NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);

