<#>
.SYNOPSIS
    Automates sqlx prepare for all department services and ingesters
.DESCRIPTION
    This script runs migrations and sqlx prepare for all tpt-gov-nz services
    that use sqlx macros. It handles common type fixes automatically.
#>

# Service configurations: @{ name = port; dbName = "..." }
$services = @(
    @{ name = "gov-dept-acc";       port = 5405; dbName = "acc" },
    @{ name = "gov-dept-caa";       port = 5406; dbName = "caa" },
    @{ name = "gov-dept-corrections"; port = 5407; dbName = "corrections" },
    @{ name = "gov-dept-crownlaw";  port = 5408; dbName = "crownlaw" },
    @{ name = "gov-dept-customs";   port = 5409; dbName = "customs" },
    @{ name = "gov-dept-defence";   port = 5410; dbName = "defence" },
    @{ name = "gov-dept-doc";       port = 5411; dbName = "doc" },
    @{ name = "gov-dept-dpmc";      port = 5412; dbName = "dpmc" },
    @{ name = "gov-dept-eqc";       port = 5413; dbName = "eqc" },
    @{ name = "gov-dept-ero";       port = 5414; dbName = "ero" },
    @{ name = "gov-dept-ethnic";    port = 5415; dbName = "ethnic" },
    @{ name = "gov-dept-fenz";      port = 5416; dbName = "fenz" },
    @{ name = "gov-dept-gcsb";      port = 5417; dbName = "gcsb" },
    @{ name = "gov-dept-hud";       port = 5418; dbName = "hud" },
    @{ name = "gov-dept-linz";      port = 5419; dbName = "linz" },
    @{ name = "gov-dept-maritime";  port = 5420; dbName = "maritime" },
    @{ name = "gov-dept-mbie";      port = 5421; dbName = "mbie" },
    @{ name = "gov-dept-mch";       port = 5422; dbName = "mch" },
    @{ name = "gov-dept-mfat";      port = 5423; dbName = "mfat" },
    @{ name = "gov-dept-mfe";       port = 5424; dbName = "mfe" },
    @{ name = "gov-dept-moe";       port = 5425; dbName = "moe" },
    @{ name = "gov-dept-moj";       port = 5426; dbName = "moj" },
    @{ name = "gov-dept-mot";       port = 5427; dbName = "mot" },
    @{ name = "gov-dept-mpi";       port = 5428; dbName = "mpi" },
    @{ name = "gov-dept-msd";       port = 5429; dbName = "msd" },
    @{ name = "gov-dept-nzdf";      port = 5430; dbName = "nzdf" },
    @{ name = "gov-dept-nzqa";      port = 5431; dbName = "nzqa" },
    @{ name = "gov-dept-nzsis";     port = 5432; dbName = "nzsis" },
    @{ name = "gov-dept-nzta";      port = 5433; dbName = "nzta" },
    @{ name = "gov-dept-oranga";    port = 5434; dbName = "oranga" },
    @{ name = "gov-dept-pacific";   port = 5435; dbName = "pacific" },
    @{ name = "gov-dept-police";    port = 5436; dbName = "police" },
    @{ name = "gov-dept-publicservice"; port = 5437; dbName = "publicservice" },
    @{ name = "gov-dept-regulation"; port = 5438; dbName = "regulation" },
    @{ name = "gov-dept-retirement"; port = 5439; dbName = "retirement" },
    @{ name = "gov-dept-sfo";       port = 5440; dbName = "sfo" },
    @{ name = "gov-dept-statsnz";   port = 5441; dbName = "statsnz" },
    @{ name = "gov-dept-tearawhiti"; port = 5442; dbName = "tearawhiti" },
    @{ name = "gov-dept-tec";       port = 5443; dbName = "tec" },
    @{ name = "gov-dept-tpk";       port = 5444; dbName = "tpk" },
    @{ name = "gov-dept-treasury";  port = 5445; dbName = "treasury" },
    @{ name = "gov-dept-women";     port = 5446; dbName = "women" },
    @{ name = "gov-dept-worksafe";  port = 5447; dbName = "worksafe" }
)