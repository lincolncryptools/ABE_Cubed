using CSV
using DataFrames

# ============================================================#
# Build CSV file from criterions internal report JSON/files   #
# ============================================================#
function read_report(base_path)
    algorithms = ["setup", "keygen", "encrypt", "decrypt"]

    println("opt,alg,dim,dim_size,policy_len,neg,neg_degree,min,max,mean,unit")
    for alg in algorithms
        path = base_path * alg
        for d in filter(f -> f != "report", readdir(path))
            full_path = "$path/$d/base/raw.csv"
            regex = r"^(opt0|opt1|opt2|opt3|opt4|opt5|opt6a|opt6b|opt6c)_(vary_attr|vary_auth|vary_lbl|vary_auth_lbl|vary_auth_attr|vary_lbl_attr)_(\d+)_(\d+)_(pos|neg)_(\d+)$"
            m = match(regex, d)
            opt = m[1]
            dim = m[2]
            dim_size = m[3]
            policy_len = m[4]
            neg = m[5]
            neg_degree = m[6]
            contents = CSV.File(full_path, normalizenames=true)
            min = minimum(contents.sample_measured_value)
            max = maximum(contents.sample_measured_value)
            mean = sum(contents.sample_measured_value) / length(contents.sample_measured_value)
            unit = contents.unit[1]
            println("$opt,$alg,$dim,$dim_size,$policy_len,$neg,$neg_degree,$min,$max,$mean,$unit")
        end
    end
end


function parseline(line)
    regex = r"test (?<descr>\S+) ... bench: \s* (?<time>.+)"
    m = match(regex, line)
    if isnothing(m)
        return nothing
    end
    descr = m["descr"]
    time = m["time"]
    regex = r"(?<alg>\w+)\/(?<opt>[[:alnum:]]+)_vary_(?<dim>auth|lbl|attr|auth_lbl|auth_attr|lbl_attr|size)_(?<len>\d+)_(?<max>\d+)_(?<neg>\d+)"
    m = match(regex, descr)
    regex = r"(?<time>\d+) (?<unit>ns)\/iter \(\+\/- (?<dev>\d+)\)"
    n = match(regex, time)
    Dict("alg" => string(m["alg"]),
         "opt" => string(m["opt"]),
         "dim" => string(m["dim"]),
         "len" => parse(Int, m["len"]),
         "max" => parse(Int, m["max"]),
         "neg" => parse(Int, m["neg"]),
         "time" => parse(Int, n["time"]),
         "unit" => string(n["unit"]),
         "dev" => parse(Int, n["dev"]) 
    )
end


# ============================================================#
# Build CSV file from criterions "bencher" output format      #
# ============================================================#
function parse_raw_txt(file)
    df = DataFrame(alg=String[], opt=String[], dim=String[], len=Int[], max=Int[], neg=Int[], time=Int[], unit=String[], dev=Int[])
    for line in readlines(file)
        dict = parseline(line)
        if !isnothing(dict)
            push!(df, dict)
        end
    end
    CSV.write(stdout, df)
end

if length(ARGS) == 0
   println("Usage: julia report_to_csv.jl <path_to_target_criterion_dir_or_raw.txt>")
   exit(5)
end
# read_report(ARGS[1])
parse_raw_txt(ARGS[1])
