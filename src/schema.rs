// @generated automatically by Diesel CLI.

pub mod upgrade {
    pub mod sql_types {
        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "auth_method", schema = "upgrade"))]
        pub struct AuthMethod;

        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "business_type", schema = "upgrade"))]
        pub struct BusinessType;

        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "effects", schema = "upgrade"))]
        pub struct Effects;

        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "last_login_method", schema = "upgrade"))]
        pub struct LastLoginMethod;

        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType, Debug)]
        #[diesel(postgres_type(name = "payment_status_enum", schema = "upgrade"))]
        pub struct PaymentStatusEnum;

        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "request_status", schema = "upgrade"))]
        pub struct RequestStatus;

        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "resources", schema = "upgrade"))]
        pub struct Resources;

        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "roles", schema = "upgrade"))]
        pub struct Roles;

        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "subsStatusEnum", schema = "upgrade"))]
        pub struct SubsStatusEnum;

        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "targets", schema = "upgrade"))]
        pub struct Targets;
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::PaymentStatusEnum;

        upgrade.chats (id) {
            id -> Int4,
            stream_uid -> Nullable<Varchar>,
            user_id -> Int4,
            cf_order_id -> Nullable<Varchar>,
            message -> Varchar,
            mark_read -> Bool,
            up_votes -> Array<Nullable<Int4>>,
            down_votes -> Array<Nullable<Int4>>,
            reply_to_id -> Nullable<Int4>,
            pinned -> Bool,
            payment_status -> PaymentStatusEnum,
            updated_at -> Nullable<Timestamp>,
            created_at -> Timestamp,
        }
    }

    diesel::table! {
        upgrade.emotes (id) {
            id -> Int4,
            #[max_length = 255]
            name -> Varchar,
            #[max_length = 255]
            code -> Varchar,
            #[max_length = 255]
            image_url -> Varchar,
            streamer_id -> Int4,
            updated_at -> Nullable<Timestamp>,
            created_at -> Timestamp,
        }
    }

    diesel::table! {
        upgrade.orders (id) {
            id -> Int4,
            payment_session_id -> Nullable<Varchar>,
            order_status -> Varchar,
            cf_order_id -> Varchar,
            user_id -> Int4,
            order_amount -> Int4,
            #[max_length = 255]
            order_currency -> Varchar,
            order_expiry_time -> Varchar,
            order_note -> Nullable<Varchar>,
            updated_at -> Nullable<Timestamp>,
            created_at -> Timestamp,
        }
    }

    diesel::table! {
        upgrade.payouts (id) {
            id -> Int4,
            user_id -> Nullable<Int4>,
            transfer_id -> Varchar,
            cf_transfer_id -> Varchar,
            status -> Varchar,
            status_code -> Varchar,
            transfer_mode -> Varchar,
            transfer_amount -> Varchar,
            transfer_service_charge -> Nullable<Varchar>,
            transfer_service_tax -> Nullable<Varchar>,
            transfer_utr -> Nullable<Varchar>,
            updated_at -> Nullable<Timestamp>,
            created_at -> Timestamp,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::Targets;
        use super::sql_types::Resources;
        use super::sql_types::Effects;

        upgrade.permissions (id) {
            id -> Int4,
            target -> Targets,
            target_id -> Int4,
            resource -> Resources,
            resource_id -> Int4,
            effect -> Nullable<Effects>,
            #[max_length = 255]
            action -> Varchar,
            updated_at -> Nullable<Timestamp>,
            created_at -> Timestamp,
        }
    }

    diesel::table! {
        upgrade.plans (id) {
            id -> Int4,
            #[max_length = 255]
            name -> Varchar,
            details -> Varchar,
            amount -> Int4,
            #[max_length = 255]
            razorpay_plan_id -> Varchar,
            streamer_id -> Int4,
            updated_at -> Nullable<Timestamp>,
            created_at -> Timestamp,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::AuthMethod;

        upgrade.session (session_id) {
            session_id -> Uuid,
            user_id -> Int4,
            auth_method -> AuthMethod,
            last_active -> Timestamp,
            invalid -> Bool,
            #[max_length = 255]
            token -> Varchar,
            #[max_length = 255]
            user_agent -> Varchar,
            #[max_length = 255]
            ip_address -> Nullable<Varchar>,
            #[max_length = 255]
            platform -> Nullable<Varchar>,
            languages -> Array<Nullable<Varchar>>,
            mobile -> Nullable<Bool>,
            expire_at -> Timestamp,
            city -> Nullable<Varchar>,
            region -> Nullable<Varchar>,
            timezone -> Nullable<Varchar>,
            telecom -> Nullable<Varchar>,
            country -> Nullable<Varchar>,
            os -> Nullable<Varchar>,
            updated_at -> Nullable<Timestamp>,
            created_at -> Timestamp,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::BusinessType;
        use super::sql_types::RequestStatus;

        upgrade.streamer_request (id) {
            id -> Int4,
            user_id -> Int4,
            #[max_length = 255]
            razorpay_account_id -> Nullable<Varchar>,
            #[max_length = 255]
            product_configuration_id -> Nullable<Varchar>,
            #[max_length = 255]
            stakeholder_id -> Nullable<Varchar>,
            account_name -> Varchar,
            account_email -> Varchar,
            dashboard_access -> Varchar,
            customer_refunds -> Varchar,
            business_name -> Varchar,
            business_type -> BusinessType,
            request_status -> RequestStatus,
            bank_ifsc_code -> Varchar,
            bank_account_number -> Varchar,
            phone_number -> Varchar,
            street_address -> Varchar,
            city -> Varchar,
            state -> Varchar,
            postal_code -> Varchar,
            pan_card -> Varchar,
            kyc_document_url -> Nullable<Varchar>,
            updated_at -> Nullable<Timestamp>,
            created_at -> Timestamp,
        }
    }

    diesel::table! {
        upgrade.streams (id) {
            id -> Int4,
            streaming_uid -> Varchar,
            stream_title -> Varchar,
            chat_slow_mode -> Nullable<Bool>,
            about -> Nullable<Varchar>,
            video_url -> Nullable<Varchar>,
            streamer_id -> Int4,
            thumbnail_url -> Varchar,
            scheduled_time -> Nullable<Timestamp>,
            is_scheduled -> Nullable<Bool>,
            end_time -> Nullable<Timestamp>,
            updated_at -> Nullable<Timestamp>,
            created_at -> Timestamp,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::SubsStatusEnum;

        upgrade.subscriptions (id) {
            id -> Int4,
            plan_id -> Int4,
            status -> SubsStatusEnum,
            #[max_length = 255]
            razorpay_subscription_id -> Varchar,
            payment_url -> Varchar,
            user_id -> Int4,
            updated_at -> Nullable<Timestamp>,
            created_at -> Timestamp,
        }
    }

    diesel::table! {
        upgrade.token_table (id) {
            id -> Int4,
            user_id -> Int4,
            user_refresh_token -> Nullable<Varchar>,
            streamer_verification_token -> Nullable<Varchar>,
            reset_password_token -> Nullable<Varchar>,
            reset_password_token_expiry -> Nullable<Timestamp>,
            email_verification_token -> Nullable<Varchar>,
            email_verification_token_expiry -> Nullable<Timestamp>,
            updated_at -> Nullable<Timestamp>,
            created_at -> Timestamp,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::Roles;
        use super::sql_types::LastLoginMethod;

        upgrade.users (id) {
            id -> Int4,
            #[max_length = 255]
            first_name -> Varchar,
            #[max_length = 255]
            last_name -> Varchar,
            #[max_length = 255]
            username -> Varchar,
            email -> Varchar,
            profile_picture -> Nullable<Varchar>,
            #[max_length = 45]
            phone_number -> Varchar,
            password_hash -> Varchar,
            role -> Nullable<Roles>,
            email_verified -> Bool,
            refrence_id -> Nullable<Varchar>,
            watch_history -> Array<Nullable<Int4>>,
            updated_at -> Nullable<Timestamp>,
            created_at -> Timestamp,
            last_login_method -> Nullable<LastLoginMethod>,
        }
    }

    diesel::joinable!(chats -> users (user_id));
    diesel::joinable!(emotes -> users (streamer_id));
    diesel::joinable!(orders -> users (user_id));
    diesel::joinable!(payouts -> users (user_id));
    diesel::joinable!(plans -> users (streamer_id));
    diesel::joinable!(session -> users (user_id));
    diesel::joinable!(streamer_request -> users (user_id));
    diesel::joinable!(streams -> users (streamer_id));
    diesel::joinable!(subscriptions -> plans (plan_id));
    diesel::joinable!(subscriptions -> users (user_id));
    diesel::joinable!(token_table -> users (user_id));

    diesel::allow_tables_to_appear_in_same_query!(
        chats,
        emotes,
        orders,
        payouts,
        permissions,
        plans,
        session,
        streamer_request,
        streams,
        subscriptions,
        token_table,
        users,
    );
}
