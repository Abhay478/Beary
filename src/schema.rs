#![allow(non_snake_case)]
// @generated automatically by Diesel CLI.
diesel::table! {
    ZSFCHANGE (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZORDER -> Nullable<Integer>,
        ZTOKEN -> Nullable<Text>,
    }
}

diesel::table! {
    ZSFCHANGEITEM (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZITEMDELETED -> Nullable<Integer>,
        ZCHANGE -> Nullable<Integer>,
        ZITEMENTITY -> Nullable<Text>,
        ZUNIQUEIDENTIFIER -> Nullable<Text>,
    }
}

diesel::table! {
    ZSFEXTERNALCHANGES (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZITEMCHANGE -> Nullable<Integer>,
        ZITEMENTITY -> Nullable<Text>,
        ZITEMOBJECTID -> Nullable<Text>,
    }
}

diesel::table! {
    ZSFFOLDER (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZORDER -> Nullable<Integer>,
        ZPERMANENTLYDELETED -> Nullable<Integer>,
        ZSKIPSYNC -> Nullable<Integer>,
        ZPARENTFOLDER -> Nullable<Integer>,
        ZCREATIONDATE -> Nullable<Timestamp>,
        ZMODIFICATIONDATE -> Nullable<Timestamp>,
        ZTITLE -> Nullable<Text>,
        ZUNIQUEIDENTIFIER -> Nullable<Text>,
    }
}

diesel::table! {
    ZSFINTERNALCHANGES (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZITEMCHANGE -> Nullable<Integer>,
        ZCHANGETYPE -> Nullable<Text>,
        ZITEMOBJECTID -> Nullable<Text>,
    }
}

diesel::table! {
    ZSFLOCATION (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZNOTE -> Nullable<Integer>,
        ZALTITUDE -> Nullable<Float>,
        ZCOURSE -> Nullable<Float>,
        ZHORIZONTALACCURACY -> Nullable<Float>,
        ZLATITUDE -> Nullable<Float>,
        ZLONGITUDE -> Nullable<Float>,
        ZSPEED -> Nullable<Float>,
        ZVERTICALACCURACY -> Nullable<Float>,
    }
}

diesel::table! {
    ZSFNOTE (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZARCHIVED -> Nullable<Integer>,
        ZENCRYPTED -> Nullable<Integer>,
        ZHASFILES -> Nullable<Integer>,
        ZHASIMAGES -> Nullable<Integer>,
        ZHASSOURCECODE -> Nullable<Integer>,
        ZLOCKED -> Nullable<Integer>,
        ZORDER -> Nullable<Integer>,
        ZPERMANENTLYDELETED -> Nullable<Integer>,
        ZPINNED -> Nullable<Integer>,
        ZSHOWNINTODAYWIDGET -> Nullable<Integer>,
        ZSKIPSYNC -> Nullable<Integer>,
        ZTODOCOMPLETED -> Nullable<Integer>,
        ZTODOINCOMPLETED -> Nullable<Integer>,
        ZTRASHED -> Nullable<Integer>,
        ZFOLDER -> Nullable<Integer>,
        ZPASSWORD -> Nullable<Integer>,
        ZSERVERDATA -> Nullable<Integer>,
        ZARCHIVEDDATE -> Nullable<Timestamp>,
        ZCONFLICTUNIQUEIDENTIFIERDATE -> Nullable<Timestamp>,
        ZCREATIONDATE -> Nullable<Timestamp>,
        ZLOCKEDDATE -> Nullable<Timestamp>,
        ZMODIFICATIONDATE -> Nullable<Timestamp>,
        ZORDERDATE -> Nullable<Timestamp>,
        ZPINNEDDATE -> Nullable<Timestamp>,
        ZTRASHEDDATE -> Nullable<Timestamp>,
        ZCONFLICTUNIQUEIDENTIFIER -> Nullable<Text>,
        ZENCRYPTIONUNIQUEIDENTIFIER -> Nullable<Text>,
        ZLASTEDITINGDEVICE -> Nullable<Text>,
        ZSUBTITLE -> Nullable<Text>,
        ZTEXT -> Nullable<Text>,
        ZTITLE -> Nullable<Text>,
        ZUNIQUEIDENTIFIER -> Nullable<Text>,
        ZENCRYPTEDDATA -> Nullable<Binary>,
        ZVECTORCLOCK -> Nullable<Binary>,
    }
}

diesel::table! {
    ZSFNOTEFILE (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZDOWNLOADED -> Nullable<Integer>,
        ZFILESIZE -> Nullable<Integer>,
        ZINDEX -> Nullable<Integer>,
        ZPERMANENTLYDELETED -> Nullable<Integer>,
        ZSKIPSYNC -> Nullable<Integer>,
        ZUNUSED -> Nullable<Integer>,
        ZUPLOADED -> Nullable<Integer>,
        ZNOTE -> Nullable<Integer>,
        ZSERVERDATA -> Nullable<Integer>,
        ZANIMATED -> Nullable<Integer>,
        ZHEIGHT -> Nullable<Integer>,
        ZWIDTH -> Nullable<Integer>,
        ZDURATION -> Nullable<Integer>,
        ZHEIGHT1 -> Nullable<Integer>,
        ZWIDTH1 -> Nullable<Integer>,
        ZCREATIONDATE -> Nullable<Timestamp>,
        ZMODIFICATIONDATE -> Nullable<Timestamp>,
        ZUPLOADEDDATE -> Nullable<Timestamp>,
        ZFILENAME -> Nullable<Text>,
        ZLASTEDITINGDEVICE -> Nullable<Text>,
        ZNORMALIZEDFILEEXTENSION -> Nullable<Text>,
        ZUNIQUEIDENTIFIER -> Nullable<Text>,
    }
}

diesel::table! {
    ZSFNOTEFILESERVERDATA (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZFILE -> Nullable<Integer>,
        Z8_FILE -> Nullable<Integer>,
        ZSYSTEMFIELDS -> Nullable<Binary>,
    }
}

diesel::table! {
    ZSFNOTESERVERDATA (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZNOTE -> Nullable<Integer>,
        ZSYSTEMFIELDS -> Nullable<Binary>,
    }
}

diesel::table! {
    ZSFNOTETAG (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZMODIFICATIONDATE -> Nullable<Timestamp>,
        ZTITLE -> Nullable<Text>,
    }
}

diesel::table! {
    ZSFPASSWORD (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZBIOMETRY -> Nullable<Integer>,
        ZENCRYPTIONVERSION -> Nullable<Integer>,
        ZCREATIONDATE -> Nullable<Timestamp>,
        ZCREATIONDEVICE -> Nullable<Text>,
        ZUNIQUEIDENTIFIER -> Nullable<Text>,
        ZENCRYPTEDDATA -> Nullable<Binary>,
        ZHINT -> Nullable<Binary>,
    }
}

diesel::table! {
    ZSFSERVERMETADATA (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZNOTEZONESUBSCRIPTIONID -> Nullable<Text>,
        ZNOTEZONEIDDATA -> Nullable<Binary>,
        ZSERVERCHANGETOKENDATA -> Nullable<Binary>,
    }
}

diesel::table! {
    ZSFSTATICNOTE (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZORDER -> Nullable<Integer>,
    }
}

diesel::table! {
    ZSFTODO (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZORDER -> Nullable<Integer>,
        ZNOTE -> Nullable<Integer>,
        ZCOMPLETIONDATE -> Nullable<Timestamp>,
        ZCREATIONDATE -> Nullable<Timestamp>,
        ZMODIFICATIONDATE -> Nullable<Timestamp>,
        ZTEXT -> Nullable<Text>,
        ZUNIQUEIDENTIFIER -> Nullable<Text>,
    }
}

diesel::table! {
    ZSFURL (Z_PK) {
        Z_PK -> Nullable<Integer>,
        Z_ENT -> Nullable<Integer>,
        Z_OPT -> Nullable<Integer>,
        ZNOTE -> Nullable<Integer>,
        ZCREATIONDATE -> Nullable<Timestamp>,
        ZMODIFICATIONDATE -> Nullable<Timestamp>,
        ZOGDESCRIPTION -> Nullable<Text>,
        ZOGIMAGE -> Nullable<Text>,
        ZOGTITLE -> Nullable<Text>,
        ZOGTYPE -> Nullable<Text>,
        ZOGURL -> Nullable<Text>,
        ZOGVIDEO -> Nullable<Text>,
        ZTITLE -> Nullable<Text>,
        ZUNIQUEIDENTIFIER -> Nullable<Text>,
        ZURL -> Nullable<Text>,
    }
}

diesel::table! {
    Z_7LINKEDNOTES (Z_7LINKEDBYNOTES, Z_7LINKEDNOTES_) {
        Z_7LINKEDBYNOTES -> Nullable<Integer>,
        #[sql_name = "Z_7LINKEDNOTES"]
        Z_7LINKEDNOTES_ -> Nullable<Integer>,
    }
}

diesel::table! {
    Z_7TAGS (Z_7NOTES, Z_14TAGS) {
        Z_7NOTES -> Nullable<Integer>,
        Z_14TAGS -> Nullable<Integer>,
    }
}

diesel::table! {
    Z_METADATA (Z_VERSION) {
        Z_VERSION -> Nullable<Integer>,
        Z_UUID -> Nullable<Text>,
        Z_PLIST -> Nullable<Binary>,
    }
}

diesel::table! {
    Z_PRIMARYKEY (Z_ENT) {
        Z_ENT -> Nullable<Integer>,
        Z_NAME -> Nullable<Text>,
        Z_SUPER -> Nullable<Integer>,
        Z_MAX -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    ZSFCHANGE,
    ZSFCHANGEITEM,
    ZSFEXTERNALCHANGES,
    ZSFFOLDER,
    ZSFINTERNALCHANGES,
    ZSFLOCATION,
    ZSFNOTE,
    ZSFNOTEFILE,
    ZSFNOTEFILESERVERDATA,
    ZSFNOTESERVERDATA,
    ZSFNOTETAG,
    ZSFPASSWORD,
    ZSFSERVERMETADATA,
    ZSFSTATICNOTE,
    ZSFTODO,
    ZSFURL,
    Z_7LINKEDNOTES,
    Z_7TAGS,
    Z_METADATA,
    Z_PRIMARYKEY,
);
