use crate::http::status::*;

pub fn status_text_id(code: u16) -> &'static str {
    match code {
        // HTTP Status 1xx
        STATUS_CONTINUE => "Lanjutkan",
        STATUS_SWITCHING_PROTOCOLS => "Beralih Protokol",
        STATUS_PROCESSING => "Processing",
        STATUS_EARLY_HINTS => "Petunjuk Awal",

        // HTTP Status 2xx
        STATUS_OK => "OK",
        STATUS_CREATED => "Dibuat",
        STATUS_ACCEPTED => "Diterima",
        STATUS_NON_AUTHORITATIVE_INFO => "Informasi Non-Resmi",
        STATUS_NO_CONTENT => "Tanpa Konten",
        STATUS_RESET_CONTENT => "Setel Ulang Konten",
        STATUS_PARTIAL_CONTENT => "Konten Sebagian",
        STATUS_MULTI_STATUS => "Multi-Status",
        STATUS_ALREADY_REPORTED => "Sudah Dilaporkan",
        STATUS_IM_USED => "IM Used",

        // HTTP Status 3xx
        STATUS_MULTIPLE_CHOICES => "Pilihan Ganda",
        STATUS_MOVED_PERMANENTLY => "Dipindahkan Permanen",
        STATUS_FOUND => "Ditemukan",
        STATUS_SEE_OTHER => "Lihat Lainnya",
        STATUS_NOT_MODIFIED => "Tidak Dimodifikasi",
        STATUS_USE_PROXY => "Gunakan Proxy",
        STATUS_TEMPORARY_REDIRECT => "Pengalihan Sementara",
        STATUS_PERMANENT_REDIRECT => "Pengalihan Permanen",

        // HTTP Status 4xx
        STATUS_BAD_REQUEST => "Bad Request",
        STATUS_UNAUTHORIZED => "Tidak Diperbolehkan",
        STATUS_PAYMENT_REQUIRED => "Payment Required",
        STATUS_FORBIDDEN => "Terlarang",
        STATUS_NOT_FOUND => "Tidak Ditemukan",
        STATUS_METHOD_NOT_ALLOWED => "Metode Tidak Diizinkan",
        STATUS_NOT_ACCEPTABLE => "Tidak Dapat Diterima",
        STATUS_PROXY_AUTH_REQUIRED => "Diperlukan Otentikasi Proksi",
        STATUS_REQUEST_TIMEOUT => "Request Timeout",
        STATUS_CONFLICT => "Konflik",
        STATUS_GONE => "Hilang",
        STATUS_LENGTH_REQUIRED => "Panjang Diperlukan",
        STATUS_PRECONDITION_FAILED => "Precondition Failed",
        STATUS_REQUEST_ENTITY_TOO_LARGE => "Payload Terlalu Besar",
        STATUS_REQUEST_URI_TOO_LONG => "URI Terlalu Panjang",
        STATUS_UNSUPPORTED_MEDIA_TYPE => "Jenis Media Yang Tidak Didukung",
        STATUS_REQUESTED_RANGE_NOT_SATISFIABLE => "Requested Range Not Statisfiable",
        STATUS_EXPECTATION_FAILED => "Expectation Failed",
        STATUS_TEAPOT => "I'm a Teapot",
        STATUS_MISDIRECTED_REQUEST => "Misdirected Request",
        STATUS_UNPROCESSABLE_ENTITY => "Unprocessable Entity",
        STATUS_LOCKED => "Locked",
        STATUS_FAILED_DEPENDENCY => "Failed Dependency",
        STATUS_TOO_EARLY => "Too Early",
        STATUS_UPGRADE_REQUIRED => "Diperlukan Peningkatan",
        STATUS_PRECONDITION_REQUIRED => "Precondition Required",
        STATUS_TOO_MANY_REQUESTS => "Too Many Requests",
        STATUS_REQUEST_HEADER_FIELDS_TOO_LARGE => "Request Header Fields Too Large",
        STATUS_UNAVAILABLE_FOR_LEGAL_REASONS => "Unavailable For Legal Reasons",

        // HTTP Status 5xx
        STATUS_INTERNAL_SERVER_ERROR => "Masalah Peladen Dalam",
        STATUS_NOT_IMPLEMENTED => "'Not Implemented",
        STATUS_BAD_GATEWAY => "Bad Gateway",
        STATUS_SERVICE_UNAVAILABLE => "Service Unavailable",
        STATUS_GATEWAY_TIMEOUT => "Gateway Timeout",
        STATUS_HTTP_VERSION_NOT_SUPPORTED => "HTTP Version Not Supported",
        STATUS_VARIANT_ALSO_NEGOTIATES => "Variant Also Negotiates",
        STATUS_INSUFFICIENT_STORAGE => "Insufficient Storage",
        STATUS_LOOP_DETECTED => "Loop Detected",
        STATUS_NOT_EXTENDED => "Not Extended",
        STATUS_NETWORK_AUTHENTICATION_REQUIRED => "Network Authentication Required",

        _ => "",
    }
}
