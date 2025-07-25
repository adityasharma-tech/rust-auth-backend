-- Your SQL goes here
CREATE SCHEMA "upgrade";
--> statement-breakpoint
CREATE TYPE "upgrade"."payment_status_enum" AS ENUM('idle', 'created', 'attempted', 'paid');--> statement-breakpoint
CREATE TYPE "upgrade"."business_type" AS ENUM('llp', 'ngo', 'individual', 'partnership', 'proprietorship', 'public_limited', 'private_limited', 'trust', 'society', 'not_yet_registered', 'educational_institutes');--> statement-breakpoint
CREATE TYPE "upgrade"."request_status" AS ENUM('pending', 'account_created', 'stakeholder_created', 'tnc_accepted', 'account_added', 'done');--> statement-breakpoint
CREATE TYPE "upgrade"."last_login_method" AS ENUM('email_password', 'sso_google', 'sso_github');--> statement-breakpoint
CREATE TYPE "upgrade"."roles" AS ENUM('streamer', 'viewer', 'admin');--> statement-breakpoint
CREATE TYPE "upgrade"."effects" AS ENUM('allow', 'disallow');--> statement-breakpoint
CREATE TYPE "upgrade"."resources" AS ENUM('stream', 'user', 'chat', 'order', 'streamer-requests');--> statement-breakpoint
CREATE TYPE "upgrade"."targets" AS ENUM('streamer', 'viewer', 'admin', 'user');--> statement-breakpoint
CREATE TYPE "upgrade"."subsStatusEnum" AS ENUM('created', 'authenticated', 'active', 'pending', 'halted', 'cancelled', 'completed', 'expired');--> statement-breakpoint
CREATE TYPE "upgrade"."auth_method" AS ENUM('email_password', 'sso_google', 'sso_github');--> statement-breakpoint
CREATE TABLE "upgrade"."chats" (
                                   "id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "upgrade"."chats_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
                                   "stream_uid" varchar,
                                   "user_id" integer NOT NULL,
                                   "cf_order_id" varchar,
                                   "message" varchar NOT NULL,
                                   "mark_read" boolean DEFAULT false NOT NULL,
                                   "up_votes" integer[] DEFAULT '{}' NOT NULL,
                                   "down_votes" integer[] DEFAULT '{}' NOT NULL,
                                   "reply_to_id" integer,
                                   "pinned" boolean DEFAULT false NOT NULL,
                                   "payment_status" "upgrade"."payment_status_enum" DEFAULT 'idle' NOT NULL,
                                   "updated_at" timestamp,
                                   "created_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "upgrade"."orders" (
                                    "id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "upgrade"."orders_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
                                    "payment_session_id" varchar,
                                    "order_status" varchar DEFAULT 'PENDING' NOT NULL,
                                    "cf_order_id" varchar NOT NULL,
                                    "user_id" integer NOT NULL,
                                    "order_amount" integer NOT NULL,
                                    "order_currency" varchar(255) DEFAULT 'INR' NOT NULL,
                                    "order_expiry_time" varchar NOT NULL,
                                    "order_note" varchar,
                                    "updated_at" timestamp,
                                    "created_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "upgrade"."payouts" (
                                     "id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "upgrade"."payouts_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
                                     "user_id" integer,
                                     "transfer_id" varchar NOT NULL,
                                     "cf_transfer_id" varchar NOT NULL,
                                     "status" varchar NOT NULL,
                                     "status_code" varchar NOT NULL,
                                     "transfer_mode" varchar NOT NULL,
                                     "transfer_amount" varchar NOT NULL,
                                     "transfer_service_charge" varchar,
                                     "transfer_service_tax" varchar,
                                     "transfer_utr" varchar,
                                     "updated_at" timestamp,
                                     "created_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "upgrade"."streams" (
                                     "id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "upgrade"."streams_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
                                     "streaming_uid" varchar NOT NULL,
                                     "stream_title" varchar NOT NULL,
                                     "chat_slow_mode" boolean DEFAULT false,
                                     "about" varchar DEFAULT '',
                                     "video_url" varchar,
                                     "streamer_id" integer NOT NULL,
                                     "thumbnail_url" varchar NOT NULL,
                                     "scheduled_time" timestamp,
                                     "is_scheduled" boolean DEFAULT false,
                                     "end_time" timestamp,
                                     "updated_at" timestamp,
                                     "created_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "upgrade"."streamer_request" (
                                              "id" integer PRIMARY KEY GENERATED BY DEFAULT AS IDENTITY (sequence name "upgrade"."streamer_request_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
                                              "user_id" integer NOT NULL,
                                              "razorpay_account_id" varchar(255),
                                              "product_configuration_id" varchar(255),
                                              "stakeholder_id" varchar(255),
                                              "account_name" varchar NOT NULL,
                                              "account_email" varchar NOT NULL,
                                              "dashboard_access" varchar DEFAULT '0' NOT NULL,
                                              "customer_refunds" varchar DEFAULT '0' NOT NULL,
                                              "business_name" varchar NOT NULL,
                                              "business_type" "upgrade"."business_type" DEFAULT 'individual' NOT NULL,
                                              "request_status" "upgrade"."request_status" DEFAULT 'pending' NOT NULL,
                                              "bank_ifsc_code" varchar NOT NULL,
                                              "bank_account_number" varchar NOT NULL,
                                              "phone_number" varchar NOT NULL,
                                              "street_address" varchar NOT NULL,
                                              "city" varchar NOT NULL,
                                              "state" varchar NOT NULL,
                                              "postal_code" varchar NOT NULL,
                                              "pan_card" varchar NOT NULL,
                                              "kyc_document_url" varchar,
                                              "updated_at" timestamp,
                                              "created_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "upgrade"."token_table" (
                                         "id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "upgrade"."token_table_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
                                         "user_id" integer NOT NULL,
                                         "user_refresh_token" varchar,
                                         "streamer_verification_token" varchar,
                                         "reset_password_token" varchar,
                                         "reset_password_token_expiry" timestamp DEFAULT '2025-07-12 17:14:03.631',
                                         "email_verification_token" varchar,
                                         "email_verification_token_expiry" timestamp DEFAULT '2025-07-12 17:14:03.631',
                                         "updated_at" timestamp,
                                         "created_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "upgrade"."users" (
                                   "id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "upgrade"."users_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
                                   "first_name" varchar(255) NOT NULL,
                                   "last_name" varchar(255) NOT NULL,
                                   "username" varchar(255) NOT NULL,
                                   "email" varchar NOT NULL,
                                   "profile_picture" varchar,
                                   "phone_number" varchar(45) NOT NULL,
                                   "password_hash" varchar NOT NULL,
                                   "role" "upgrade"."roles" DEFAULT 'viewer',
                                   "email_verified" boolean DEFAULT false NOT NULL,
                                   "refrence_id" varchar,
                                   "watch_history" integer[] DEFAULT ARRAY[]::integer[] NOT NULL,
                                   "updated_at" timestamp,
                                   "created_at" timestamp DEFAULT now() NOT NULL,
                                   "last_login_method" "upgrade"."last_login_method" DEFAULT 'email_password',
                                   CONSTRAINT "users_username_unique" UNIQUE("username"),
                                   CONSTRAINT "users_email_unique" UNIQUE("email")
);
--> statement-breakpoint
CREATE TABLE "upgrade"."permissions" (
                                         "id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "upgrade"."permissions_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
                                         "target" "upgrade"."targets" NOT NULL,
                                         "target_id" integer NOT NULL,
                                         "resource" "upgrade"."resources" NOT NULL,
                                         "resource_id" integer NOT NULL,
                                         "effect" "upgrade"."effects" DEFAULT 'allow',
                                         "action" varchar(255) NOT NULL,
                                         "updated_at" timestamp,
                                         "created_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "upgrade"."plans" (
                                   "id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "upgrade"."plans_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
                                   "name" varchar(255) NOT NULL,
                                   "details" varchar NOT NULL,
                                   "amount" integer NOT NULL,
                                   "razorpay_plan_id" varchar(255) DEFAULT '' NOT NULL,
                                   "streamer_id" integer NOT NULL,
                                   "updated_at" timestamp,
                                   "created_at" timestamp DEFAULT now() NOT NULL,
                                   CONSTRAINT "plans_razorpayPlanId_unique" UNIQUE("razorpay_plan_id"),
                                   CONSTRAINT "plans_streamerId_unique" UNIQUE("streamer_id")
);
--> statement-breakpoint
CREATE TABLE "upgrade"."subscriptions" (
                                           "id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "upgrade"."subscriptions_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
                                           "plan_id" integer NOT NULL,
                                           "status" "upgrade"."subsStatusEnum" NOT NULL,
                                           "razorpay_subscription_id" varchar(255) NOT NULL,
                                           "payment_url" varchar NOT NULL,
                                           "user_id" integer NOT NULL,
                                           "updated_at" timestamp,
                                           "created_at" timestamp DEFAULT now() NOT NULL,
                                           CONSTRAINT "subscriptions_razorpaySubscriptionId_unique" UNIQUE("razorpay_subscription_id")
);
--> statement-breakpoint
CREATE TABLE "upgrade"."emotes" (
                                    "id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "upgrade"."emotes_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
                                    "name" varchar(255) NOT NULL,
                                    "code" varchar(255) NOT NULL,
                                    "image_url" varchar(255) NOT NULL,
                                    "streamer_id" integer NOT NULL,
                                    "updated_at" timestamp,
                                    "created_at" timestamp DEFAULT now() NOT NULL,
                                    CONSTRAINT "emotes_code_unique" UNIQUE("code")
);
--> statement-breakpoint
CREATE TABLE "upgrade"."session" (
                                     "session_id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
                                     "user_id" integer NOT NULL,
                                     "auth_method" "upgrade"."auth_method" DEFAULT 'email_password' NOT NULL,
                                     "last_active" timestamp DEFAULT now() NOT NULL,
                                     "invalid" boolean DEFAULT false NOT NULL,
                                     "token" varchar(255) NOT NULL,
                                     "user_agent" varchar(255) NOT NULL,
                                     "ip_address" varchar(255),
                                     "platform" varchar(255),
                                     "languages" varchar(255)[] DEFAULT '{}' NOT NULL,
                                     "mobile" boolean DEFAULT false,
                                     "expire_at" timestamp NOT NULL,
                                     "city" varchar,
                                     "region" varchar,
                                     "timezone" varchar,
                                     "telecom" varchar,
                                     "country" varchar,
                                     "os" varchar,
                                     "updated_at" timestamp,
                                     "created_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
ALTER TABLE "upgrade"."chats" ADD CONSTRAINT "chats_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "upgrade"."users"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "upgrade"."orders" ADD CONSTRAINT "orders_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "upgrade"."users"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "upgrade"."payouts" ADD CONSTRAINT "payouts_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "upgrade"."users"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "upgrade"."streams" ADD CONSTRAINT "streams_streamer_id_users_id_fk" FOREIGN KEY ("streamer_id") REFERENCES "upgrade"."users"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "upgrade"."streamer_request" ADD CONSTRAINT "streamer_request_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "upgrade"."users"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "upgrade"."token_table" ADD CONSTRAINT "token_table_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "upgrade"."users"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "upgrade"."plans" ADD CONSTRAINT "plans_streamer_id_users_id_fk" FOREIGN KEY ("streamer_id") REFERENCES "upgrade"."users"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "upgrade"."subscriptions" ADD CONSTRAINT "subscriptions_plan_id_plans_id_fk" FOREIGN KEY ("plan_id") REFERENCES "upgrade"."plans"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "upgrade"."subscriptions" ADD CONSTRAINT "subscriptions_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "upgrade"."users"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "upgrade"."emotes" ADD CONSTRAINT "emotes_streamer_id_users_id_fk" FOREIGN KEY ("streamer_id") REFERENCES "upgrade"."users"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "upgrade"."session" ADD CONSTRAINT "session_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "upgrade"."users"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
CREATE INDEX "streamUidIdx" ON "upgrade"."chats" USING btree ("stream_uid");--> statement-breakpoint
CREATE INDEX "cfOrderIdIdx" ON "upgrade"."orders" USING btree ("cf_order_id");--> statement-breakpoint
CREATE UNIQUE INDEX "streamingUidIdx" ON "upgrade"."streams" USING btree ("streaming_uid");--> statement-breakpoint
CREATE UNIQUE INDEX "razorpayAccountIdIdx" ON "upgrade"."streamer_request" USING btree ("razorpay_account_id");--> statement-breakpoint
CREATE INDEX "accountEmailIdx" ON "upgrade"."streamer_request" USING btree ("account_email");--> statement-breakpoint
CREATE UNIQUE INDEX "emailIdx" ON "upgrade"."users" USING btree ("email");--> statement-breakpoint
CREATE UNIQUE INDEX "usernameIdx" ON "upgrade"."users" USING btree ("username");--> statement-breakpoint
CREATE UNIQUE INDEX "wholeIndex" ON "upgrade"."permissions" USING btree ("target","target_id","resource","resource_id","effect","action");--> statement-breakpoint
CREATE INDEX "planIdIndex" ON "upgrade"."subscriptions" USING btree ("plan_id");--> statement-breakpoint
CREATE INDEX "userIdIndex" ON "upgrade"."subscriptions" USING btree ("user_id");--> statement-breakpoint
CREATE UNIQUE INDEX "codeIndex" ON "upgrade"."emotes" USING btree ("code");--> statement-breakpoint
CREATE INDEX "userIndex" ON "upgrade"."emotes" USING btree ("streamer_id");--> statement-breakpoint
CREATE UNIQUE INDEX "session_token_index" ON "upgrade"."session" USING btree ("token");