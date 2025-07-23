using DataFrames
using Plots
using CSV

# ============================================================================#
# Plot all optimizations per dimension with fixed negation degree (line plot) #
# ============================================================================#
function plot_opts_over_dim_line_single(df, alg, dim, neg, max, opts)
    df = filter(row -> row.alg == alg, df)
    df = filter(row -> row.dim == dim, df)
    df = filter(row -> row.neg == neg, df)
    df = filter(row -> row.max == max, df)
    p = plot()
    for opt in opts
        df2 = filter(row -> row.opt == opt, df)
        plot!(p, df2.len, df2.time,
            label=opt,
            title=alg,
            xticks=df2.len)
    end
    dim_to_x_label = Dict([
        ("auth", "authorities"),
        ("lbl", "labels"),
        ("attr", "attributes"),
        ("auth_attr", "(authority, attribute)s"),
        ("auth_lbl", "(authority, label)s"),
        ("lbl_attr", "(label, attributes)s")])
    xlabel!(p, dim_to_x_label[dim])
    ylabel!(p, "time [ns]")
    return p
end

function plot_opts_over_dim_line(df, dim, neg, max, opts)
    setup = plot_opts_over_dim_line_single(df, "setup", dim, neg, max, opts)
    keygen = plot_opts_over_dim_line_single(df, "keygen", dim, neg, max, opts)
    encrypt = plot_opts_over_dim_line_single(df, "encrypt", dim, neg, max, opts)
    decrypt = plot_opts_over_dim_line_single(df, "decrypt", dim, neg, max, opts)
    title = "dim = $dim, neg_degree = $neg, policy_len = $max"
    p = plot(setup, keygen, encrypt, decrypt,
        layout = (2, 2),
        size=(1200, 1200),
        plot_title=title)
    return p
end


# ===========================================================================#
# Plot all optimizations per dimension with fixed negation degree (bar plot) #
# ===========================================================================#
function plot_opts_over_dim_bar_single(df, alg, dim, neg, max, opts)
    df = filter(row -> row.alg == alg, df)
    df = filter(row -> row.dim == dim, df)
    df = filter(row -> row.neg == neg, df)
    df = filter(row -> row.max == max, df)
    p = plot()
    for (idx, opt) in enumerate(opts)
        df2 = filter(row -> row.opt == opt, df)
        bar!(p, [(idx - 1) + 10 * x for x in 1:10], df2.time,
            label=opt,
            bar_width=1,
            title=alg,
            xticks=([(idx - 1) + 10 * x + 7 for x in 0:10], [1, 2, 3, 4, 5, 6, 10, 12, 30, 60]),
            )
    end
    dim_to_x_label = Dict([
        ("auth", "authorities"),
        ("lbl", "labels"),
        ("attr", "attributes"),
        ("auth_attr", "(authority, attribute)s"),
        ("auth_lbl", "(authority, label)s"),
        ("lbl_attr", "(label, attributes)s")])
    xlabel!(p, dim_to_x_label[dim])
    ylabel!(p, "time [ns]")
    return p
end

function plot_opts_over_dim_bar(df, dim, neg, max, opts)
    setup = plot_opts_over_dim_bar_single(df, "setup", dim, neg, max, opts)
    keygen = plot_opts_over_dim_bar_single(df, "keygen", dim, neg, max, opts)
    encrypt = plot_opts_over_dim_bar_single(df, "encrypt", dim, neg, max, opts)
    decrypt = plot_opts_over_dim_bar_single(df, "decrypt", dim, neg, max, opts)
    title = "dim = $dim, neg_degree = $neg, policy_len = $max"
    p = plot(setup, keygen, encrypt, decrypt,
        layout = (2, 2),
        size=(1200, 1200),
        plot_title=title)
    return p
end


# ============================================================================#
# Plot all negation degrees per dimension with fixed optimization (line plot) #
# ============================================================================#
function plot_neg_degrees_over_dim_line_single(df, alg, dim, opt, max, negs)
    df = filter(row -> row.alg == alg, df)
    df = filter(row -> row.dim == dim, df)
    df = filter(row -> row.opt == opt, df)
    df = filter(row -> row.max == max, df)
    p = plot()
    for neg in negs
        df2 = filter(row -> row.neg == neg, df)
        plot!(p, df2.len, df2.time,
            label="degree = $neg",
            title=alg,
            xticks=df2.len)
    end
    dim_to_x_label = Dict([
        ("auth", "authorities"),
        ("lbl", "labels"),
        ("attr", "attributes"),
        ("auth_attr", "(authority, attribute)s"),
        ("auth_lbl", "(authority, label)s"),
        ("lbl_attr", "(label, attributes)s")])
    xlabel!(p, dim_to_x_label[dim])
    ylabel!(p, "time [ns]")
    return p
end

function plot_neg_degrees_over_dim_line(df, dim, opt, max, negs)
    setup = plot_neg_degrees_over_dim_line_single(df, "setup", dim, opt, max, negs)
    keygen = plot_neg_degrees_over_dim_line_single(df, "keygen", dim, opt, max, negs)
    encrypt = plot_neg_degrees_over_dim_line_single(df, "encrypt", dim, opt, max, negs)
    decrypt = plot_neg_degrees_over_dim_line_single(df, "decrypt", dim, opt, max, negs)
    title = "dim = $dim, variant = $opt, policy_len = $max"
    p = plot(setup, keygen, encrypt, decrypt,
        layout = (2, 2),
        size=(1200, 1200),
        plot_title=title)
    return p
end


# =================================================================================#
# Plot all optimizations over policy length with fixed negation degree (line plot) #
# =================================================================================#
function plot_opts_over_size_line_single(df, alg, dim, neg, max, opts)
    df = filter(row -> row.alg == alg, df)
    df = filter(row -> row.dim == dim, df)
    df = filter(row -> row.neg == neg, df)
    df = filter(row -> row.max == max, df)
    p = plot()
    for opt in opts
        df2 = filter(row -> row.opt == opt, df)
        plot!(p, df2.len, df2.time,
            label=opt,
            title=alg,
            xticks=0:10:101)
    end
    dim_to_x_label = Dict([
        ("auth", "authorities"),
        ("lbl", "labels"),
        ("attr", "attributes"),
        ("auth_attr", "(authority, attribute)s"),
        ("auth_lbl", "(authority, label)s"),
        ("lbl_attr", "(label, attributes)s"),
        ("size", "policy size")])
    xlabel!(p, dim_to_x_label[dim])
    ylabel!(p, "time [ns]")
    return p
end

function plot_opts_over_size_line(df, dim, neg, max, opts)
    setup = plot_opts_over_size_line_single(df, "setup", dim, neg, max, opts)
    keygen = plot_opts_over_size_line_single(df, "keygen", dim, neg, max, opts)
    encrypt = plot_opts_over_size_line_single(df, "encrypt", dim, neg, max, opts)
    decrypt = plot_opts_over_size_line_single(df, "decrypt", dim, neg, max, opts)
    title = "dim = $dim, neg_degree = $neg, all_unique"
    p = plot(setup, keygen, encrypt, decrypt,
        layout = (2, 2),
        size=(1200, 1200),
        plot_title=title)
    return p
end


# ================================================================================#
# Plot all optimizations over policy length with fixed negation degree (bar plot) #
# ================================================================================#
function plot_opts_over_size_bar_single(df, alg, dim, neg, max, opts)
    df = filter(row -> row.alg == alg, df)
    df = filter(row -> row.dim == dim, df)
    df = filter(row -> row.neg == neg, df)
    df = filter(row -> row.max == max, df)
    p = plot()
    for (idx, opt) in enumerate(opts)
        df2 = filter(row -> row.opt == opt, df)
        base = collect(5:5:100)
        pushfirst!(base, 1)
        xs = [(idx - 1) + 10 * x for x in 1:21]
        ys = [df2.time[x] for x in base]
        bar!(p, xs, ys,
            label=opt,
            bar_width=1,
            title=alg,
            xticks=([(idx - 2) + 10 * x + 7 for x in 0:20], base),
            )
    end
    dim_to_x_label = Dict([
        ("auth", "authorities"),
        ("lbl", "labels"),
        ("attr", "attributes"),
        ("auth_attr", "(authority, attribute)s"),
        ("auth_lbl", "(authority, label)s"),
        ("lbl_attr", "(label, attributes)s"),
        ("size", "policy size")])
    xlabel!(p, dim_to_x_label[dim])
    ylabel!(p, "time [ns]")
    return p
end

function plot_opts_over_size_bar(df, dim, neg, max, opts)
    setup = plot_opts_over_size_bar_single(df, "setup", dim, neg, max, opts)
    keygen = plot_opts_over_size_bar_single(df, "keygen", dim, neg, max, opts)
    encrypt = plot_opts_over_size_bar_single(df, "encrypt", dim, neg, max, opts)
    decrypt = plot_opts_over_size_bar_single(df, "decrypt", dim, neg, max, opts)
    title = "dim = $dim, neg_degree = $neg, policy_len = $max"
    # p = plot(setup,
    #     plot_title=title,
    #     layout = (1, 1),
    #     size=(1200, 1200))
    p = plot(setup, keygen, encrypt, decrypt,
        layout = (2, 2),
        size=(1200, 1200),
        plot_title=title)
    return p
end


# =================================================================================#
# Plot all negation degrees over policy length with fixed optimization (line plot) #
# =================================================================================#
function plot_neg_degrees_over_size_line_single(df, alg, dim, opt, max, negs)
    df = filter(row -> row.alg == alg, df)
    df = filter(row -> row.dim == dim, df)
    df = filter(row -> row.opt == opt, df)
    df = filter(row -> row.max == max, df)
    p = plot()
    for neg in negs
        df2 = filter(row -> row.neg == neg, df)
        plot!(p, df2.len, df2.time,
            label="degree = $neg",
            title=alg,
            xticks=0:10:101)
    end
    dim_to_x_label = Dict([
        ("auth", "authorities"),
        ("lbl", "labels"),
        ("attr", "attributes"),
        ("auth_attr", "(authority, attribute)s"),
        ("auth_lbl", "(authority, label)s"),
        ("lbl_attr", "(label, attributes)s"),
        ("size", "policy size")])
    xlabel!(p, dim_to_x_label[dim])
    ylabel!(p, "time [ns]")
    return p
end

function plot_neg_degrees_over_size_line(df, dim, opt, max, negs)
    setup = plot_neg_degrees_over_size_line_single(df, "setup", dim, opt, max, negs)
    keygen = plot_neg_degrees_over_size_line_single(df, "keygen", dim, opt, max, negs)
    encrypt = plot_neg_degrees_over_size_line_single(df, "encrypt", dim, opt, max, negs)
    decrypt = plot_neg_degrees_over_size_line_single(df, "decrypt", dim, opt, max, negs)
    title = "dim = $dim, variant = $opt, policy_len = $max"
    p = plot(setup, keygen, encrypt, decrypt,
        layout = (2, 2),
        size=(1200, 1200),
        plot_title=title)
    return p
end


# ======================================================================#
# Plot all optimizations over neg degrees with fixed length (line plot) #
# ======================================================================#
function plot_opts_over_neg_degrees_single(df, alg, dim, opts, max)
    df = filter(row -> row.alg == alg, df)
    df = filter(row -> row.dim == dim, df)
    df = filter(row -> row.max == max, df)
    p = plot()
    for opt in opts
        df2 = filter(row -> row.opt == opt, df)
        plot!(p, df2.neg, df2.time,
            label=opt,
            title=alg,
            xticks=[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 30])
    end
    xlabel!(p, "negation degree")
    ylabel!(p, "time [ns]")
    return p
end

function plot_opts_over_neg_degrees(df, dim, max, opts)
    setup = plot_opts_over_neg_degrees_single(df, "setup", dim, opts, max)
    keygen = plot_opts_over_neg_degrees_single(df, "keygen", dim, opts, max)
    encrypt = plot_opts_over_neg_degrees_single(df, "encrypt", dim, opts, max)
    decrypt = plot_opts_over_neg_degrees_single(df, "decrypt", dim, opts, max)
    title = "dim = $dim, policy_len = $max"
    p = plot(setup, keygen, encrypt, decrypt,
        layout = (2, 2),
        size=(1200, 1200),
        plot_title=title)
    return p
end


function plot_strat_01a()
    file = "./revision/strat01a/data.csv"
    df = DataFrame(CSV.File(file))

    MAX = 100
    negs = [0, 1, 5, 10]
    dims = ["size"]
    opts = ["opt0", "opt1", "opt2", "opt3", "opt4", "opt5", "opt6"]
    algs = ["setup", "keygen", "encrypt", "decrypt"]

    for neg in negs
        for dim in dims
            p = plot_opts_over_size_line(df, dim, neg, MAX, opts)
            savefig(p, "./revision/strat01a/plots/over_size/line/$(dim)_$(MAX)_$(neg).svg")
        end
    end

    for neg in negs
        for dim in dims
            for alg in algs
                p = plot_opts_over_size_bar_single(df, alg, dim, neg, MAX, opts)
                savefig(p, "./revision/strat01a/plots/over_size/bar/$(alg)_$(dim)_$(MAX)_$neg.svg")
            end
            p = plot_opts_over_size_bar(df, dim, neg, MAX, opts)
            savefig(p, "./revision/strat01a/plots/over_size/bar/$(dim)_$(MAX)_$(neg).svg")
        end
    end

    for dim in dims
        for opt in opts
            p = plot_neg_degrees_over_size_line(df, dim, opt, MAX, negs)
            savefig(p, "./revision/strat01a/plots/over_neg_degree/$(dim)_$(MAX)_$opt.svg")
        end
    end
end


function plot_strat_01b()
    file = "./revision/strat01b/data.csv"
    df = DataFrame(CSV.File(file))

    MAX = 50
    opts = ["opt0", "opt1", "opt2", "opt3", "opt4", "opt5", "opt6"]

    p = plot_opts_over_neg_degrees(df, "size", MAX, opts)
    savefig(p, "./revision/strat01b/plots/over_neg_degree/line/size_$(MAX).svg")
end


function plot_strat_02()
    file = "./revision/strat02/data.csv"
    df = DataFrame(CSV.File(file))

    MAX = 60
    negs = [0, 1, 4, 7]
    dims = ["auth", "lbl", "attr", "auth_lbl", "auth_attr", "lbl_attr"]
    opts = ["opt0", "opt1", "opt2", "opt3", "opt4", "opt5", "opt6"]

    for dim in dims
        for neg in negs
            p = plot_opts_over_dim_line(df, dim, neg, MAX, opts)
            savefig(p, "./revision/strat02/plots/over_dim/line/$(dim)_$(MAX)_$(neg).svg")

            p = plot_opts_over_dim_bar(df, dim, neg, MAX, opts)
            savefig(p, "./revision/strat02/plots/over_dim/bar/$(dim)_$(MAX)_$(neg).svg")
        end
    end

    for opt in opts
        for dim in dims
            p = plot_neg_degrees_over_dim_line(df, dim, opt, MAX, negs)
            savefig(p, "./revision/strat02/plots/over_neg_degree/$(dim)_$(MAX)_$(opt).svg")
        end
    end
end


function main()
    plot_strat_01a()
    plot_strat_01b()
    plot_strat_02()
end


main()
