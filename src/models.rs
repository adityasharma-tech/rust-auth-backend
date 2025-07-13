use diesel::prelude::*;
use chrono::NaiveDateTime;
use uuid::Uuid;

pub mod upgrade {
    use serde::Deserialize;
    use crate::schema::upgrade::sql_types::{AuthMethod, BusinessType, Effects, RequestStatus, Resources, SubsStatusEnum, Targets};
    use super::*;


    #[derive(Debug, diesel_derive_enum::DbEnum)]
    #[db_enum(existing_type_path = "crate::schema::upgrade::sql_types::Roles")]
    pub enum Roles {
        Admin,
        Streamer,
        Viewer
    }

    #[derive(Debug, diesel_derive_enum::DbEnum)]
    #[db_enum(existing_type_path = "crate::schema::upgrade::sql_types::LastLoginMethod")]
    pub enum LastLoginMethod {
        EmailPassword,
        SsoGoogle,
        SsoGithub
    }

    #[derive(Debug, diesel_derive_enum::DbEnum)]
    #[db_enum(existing_type_path = "crate::schema::upgrade::sql_types::PaymentStatusEnum")]
    pub enum PaymentStatusEnum {
        Idle,
        Created,
        Attempted,
        Paid
    }

    #[derive(Queryable, Identifiable)]
    #[diesel(table_name = crate::schema::upgrade::chats)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Chat {
        pub id: i32,
        pub stream_uid: Option<String>,
        pub user_id: i32,
        pub cf_order_id: Option<String>,
        pub message: String,
        pub mark_read: bool,
        pub up_votes: Vec<Option<i32>>,
        pub down_votes: Vec<Option<i32>>,
        pub reply_to_id: Option<i32>,
        pub pinned: bool,
        pub payment_status: Option<PaymentStatusEnum>,
        pub updated_at: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
    }

    #[derive(Queryable, Identifiable)]
    #[diesel(table_name = crate::schema::upgrade::emotes)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Emote {
        pub id: i32,
        pub name: String,
        pub code: String,
        pub image_url: String,
        pub streamer_id: i32,
        pub updated_at: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
    }

    #[derive(Queryable, Identifiable)]
    #[diesel(table_name = crate::schema::upgrade::orders)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Order {
        pub id: i32,
        pub payment_session_id: Option<String>,
        pub order_status: String,
        pub cf_order_id: String,
        pub user_id: i32,
        pub order_amount: i32,
        pub order_currency: String,
        pub order_expiry_time: String,
        pub order_note: Option<String>,
        pub updated_at: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
    }

    #[derive(Debug, Queryable, Identifiable)]
    #[diesel(table_name = crate::schema::upgrade::payouts)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Payout {
        pub id: i32,
        pub user_id: Option<i32>,
        pub transfer_id: String,
        pub cf_transfer_id: String,
        pub status: String,
        pub status_code: String,
        pub transfer_mode: String,
        pub transfer_amount: String,
        pub transfer_service_charge: Option<String>,
        pub transfer_service_tax: Option<String>,
        pub transfer_utr: Option<String>,
        pub updated_at: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
    }

    #[derive(Queryable, Identifiable)]
    #[diesel(table_name = crate::schema::upgrade::permissions)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Permission {
        pub id: i32,
        pub target: Targets,
        pub target_id: i32,
        pub resource: Resources,
        pub resource_id: i32,
        pub effect: Option<Effects>,
        pub action: String,
        pub updated_at: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
    }

    #[derive(Debug, Queryable, Identifiable)]
    #[diesel(table_name = crate::schema::upgrade::plans)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Plan {
        pub id: i32,
        pub name: String,
        pub details: String,
        pub amount: i32,
        pub razorpay_plan_id: String,
        pub streamer_id: i32,
        pub updated_at: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
    }

    #[derive(Queryable, Identifiable)]
    #[diesel(table_name = crate::schema::upgrade::session)]
    #[diesel(primary_key(session_id))]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Session {
        pub session_id: Uuid,
        pub user_id: i32,
        pub auth_method: AuthMethod,
        pub last_active: NaiveDateTime,
        pub invalid: bool,
        pub token: String,
        pub user_agent: String,
        pub ip_address: Option<String>,
        pub platform: Option<String>,
        pub languages: Vec<Option<String>>,
        pub mobile: Option<bool>,
        pub expire_at: NaiveDateTime,
        pub city: Option<String>,
        pub region: Option<String>,
        pub timezone: Option<String>,
        pub telecom: Option<String>,
        pub country: Option<String>,
        pub os: Option<String>,
        pub updated_at: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
    }

    #[derive(Queryable, Identifiable)]
    #[diesel(table_name = crate::schema::upgrade::streamer_request)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct StreamerRequest {
        pub id: i32,
        pub user_id: i32,
        pub razorpay_account_id: Option<String>,
        pub product_configuration_id: Option<String>,
        pub stakeholder_id: Option<String>,
        pub account_name: String,
        pub account_email: String,
        pub dashboard_access: String,
        pub customer_refunds: String,
        pub business_name: String,
        pub business_type: BusinessType,
        pub request_status: RequestStatus,
        pub bank_ifsc_code: String,
        pub bank_account_number: String,
        pub phone_number: String,
        pub street_address: String,
        pub city: String,
        pub state: String,
        pub postal_code: String,
        pub pan_card: String,
        pub kyc_document_url: Option<String>,
        pub updated_at: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
    }

    #[derive(Debug, Queryable, Identifiable)]
    #[diesel(table_name = crate::schema::upgrade::streams)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Stream {
        pub id: i32,
        pub streaming_uid: String,
        pub stream_title: String,
        pub chat_slow_mode: Option<bool>,
        pub about: Option<String>,
        pub video_url: Option<String>,
        pub streamer_id: i32,
        pub thumbnail_url: String,
        pub scheduled_time: Option<NaiveDateTime>,
        pub is_scheduled: Option<bool>,
        pub end_time: Option<NaiveDateTime>,
        pub updated_at: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
    }

    #[derive(Queryable, Identifiable)]
    #[diesel(table_name = crate::schema::upgrade::subscriptions)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Subscription {
        pub id: i32,
        pub plan_id: i32,
        pub status: SubsStatusEnum,
        pub razorpay_subscription_id: String,
        pub payment_url: String,
        pub user_id: i32,
        pub updated_at: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
    }

    #[derive(Debug, Queryable, Identifiable)]
    #[diesel(table_name = crate::schema::upgrade::token_table)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct TokenTable {
        pub id: i32,
        pub user_id: i32,
        pub user_refresh_token: Option<String>,
        pub streamer_verification_token: Option<String>,
        pub reset_password_token: Option<String>,
        pub reset_password_token_expiry: Option<NaiveDateTime>,
        pub email_verification_token: Option<String>,
        pub email_verification_token_expiry: Option<NaiveDateTime>,
        pub updated_at: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
    }

    #[derive(Insertable)]
    #[diesel(table_name = crate::schema::upgrade::users)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct NewUser {
        pub first_name: String,
        pub last_name: String,
        pub username: String,
        pub email: String,
        pub phone_number: String,
        pub password_hash: String,
        pub updated_at: Option<NaiveDateTime>
    }

    #[derive(Debug, Queryable, Identifiable, Selectable)]
    #[diesel(table_name = crate::schema::upgrade::users)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct User {
        pub id: i32,
        pub first_name: String,
        pub last_name: String,
        pub username: String,
        pub email: String,
        pub profile_picture: Option<String>,
        pub phone_number: String,
        pub password_hash: String,
        pub role: Option<Roles>,
        pub email_verified: bool,
        pub refrence_id: Option<String>,
        pub watch_history: Vec<Option<i32>>,
        pub updated_at: Option<NaiveDateTime>,
        pub created_at: NaiveDateTime,
        pub last_login_method: Option<LastLoginMethod>,
    }
}
