import matplotlib.pyplot as plt
import tikzplotlib


class UserAttr:
    def __init__(self, auth, lbl, attr, neg = False):
        self.auth = auth
        self.lbl = lbl
        self.attr = attr
        self.neg = neg

    def __str__(self):
        if self.neg:
            return f"{self.auth}.{self.lbl}:!{self.attr}"
        else:
            return f"{self.auth}.{self.lbl}:{self.attr}"

    def __repr__(self):
        return str(self)

    def clone(self):
        return UserAttr(self.auth, self.lbl, self.attr, self.neg)


class Policy:
    def __init__(self, negs):
        self.terms = []
        self.negs = negs

    def add(self, ua):
        neg = len(self.terms) < self.negs
        ua = UserAttr(ua.auth, ua.lbl, ua.attr, neg)
        self.terms.append(ua)

    def __str__(self):
        return " & ".join([str(t) for t in self.terms])

    def __repr__(self):
        return str(self)


class UserSet:
    def __init__(self, pol, deg):
        self.user_attrs = []
        auth_lbl_done = set()
        for t in pol.terms:
            if t.neg:
                for i in range(deg):
                    if not (t.auth, t.lbl) in auth_lbl_done:
                        ua = UserAttr(t.auth, t.lbl, f"{t.attr}_{i}")
                        self.user_attrs.append(ua)
                auth_lbl_done.add((t.auth, t.lbl))
            else:
                self.user_attrs.append(t)

    def __str__(self):
        return f"{{{', '.join([str(u) for u in self.user_attrs])}}}"

    def __repr__(self):
        return str(self)


class InputGen:
    def __init__(self, len, divs, negs, deg):
        if negs > len:
            raise Exception("Cannot have more negations than terms in policy")
        self.len = len
        self.divs = divs
        self.negs = negs
        self.deg = deg

    def distribute(self, items, groups):
        if len(items) % len(groups) != 0:
            raise Exception("Invalid combination of items/groups")
        size = int(len(items) / len(groups))
        res = []
        for i, g in enumerate(groups):
            start = i*size
            end = start + size
            res.append((g, items[start:end]))
        return res

    def flatten_auth(self, dist):
        res = []
        for x, ys in dist:
            for y0, y1, y2 in ys:
                res.append(UserAttr(x, y0, y1, y2))
        return res

    def vary_auth(self):
        res = []
        for d in self.divs:
            auths = [f"{i}" for i in range(d)]
            lbl_attrs = [(f"{i}", f"{i}", i < self.negs) for i in range(self.len)]
            dist = self.distribute(lbl_attrs, auths)
            pol = Policy(self.negs)
            for ua in self.flatten_auth(dist):
                pol.add(ua)
            uset = UserSet(pol, self.deg)
            res.append((pol, uset))
        return res

    def flatten_auth_lbl(self, dist):
        res = []
        for (x0, x1), ys in dist:
            for y in ys:
                res.append(UserAttr(x0, x1, y, False))
        return res

    def vary_auth_lbl(self):
        res = []
        for d in self.divs:
            auth_lbls = [(f"{i}", f"{i}") for i in range(d)]
            attrs = [f"{i}" for i in range(self.len)]
            dist = self.distribute(attrs, auth_lbls)
            pol = Policy(self.negs)
            for ua in self.flatten_auth_lbl(dist):
                pol.add(ua)
            uset = UserSet(pol, self.deg)
            res.append((pol, uset))
        return res

    def flatten_auth_attr(self, dist):
        res = []
        for (x0, x1), ys in dist:
            for y in ys:
                res.append(UserAttr(x0, y, x1, False))
        return res

    def vary_auth_attr(self):
        res = []
        for d in self.divs:
            auth_attrs = [(f"{i}", f"{i}") for i in range(d)]
            lbls = [f"{i}" for i in range(self.len)]
            dist = self.distribute(lbls, auth_attrs)
            pol = Policy(self.negs)
            for ua in self.flatten_auth_attr(dist):
                pol.add(ua)
            uset = UserSet(pol, self.deg)
            res.append((pol, uset))
        return res


def iota(uset):
    attrs_by_auth = dict()
    for attr in uset.user_attrs:
        x = attrs_by_auth.get(attr.auth, [])
        x.append(attr)
        attrs_by_auth[attr.auth] = x

    m = 0
    iota_storage = dict()
    for _, attrs in attrs_by_auth.items():
        attrs_by_lbl = dict()
        for attr in attrs:
            x = attrs_by_lbl.get(attr.lbl, [])
            x.append(attr.attr)
            attrs_by_lbl[attr.lbl] = x

        for lbl, attrs in attrs_by_lbl.items():
            i = 0
            for attr in attrs:
                key = (lbl, attr)
                iota_storage[key] = i
                m = max(m, i)
                i += 1

    return m + 1


def tau(uset):
    attrs_by_auth_lbl = dict()
    for attr in uset.user_attrs:
        key =  (attr.auth, attr.lbl)
        x = attrs_by_auth_lbl.get(key, [])
        x.append(attr)
        attrs_by_auth_lbl[key] = x

    m = 0
    tau_storage = dict()
    for (auth, lbl), attrs in attrs_by_auth_lbl.items():
        i = 0
        for attr in attrs:
            key = (auth, lbl, attr.attr)
            tau_storage[key] = i
            m = max(m, i)
            i += 1

    return m + 1


def tau_tilde(uset):
    attrs_by_auth = dict()
    for attr in uset.user_attrs:
        x = attrs_by_auth.get(attr.auth, [])
        x.append(attr)
        attrs_by_auth[attr.auth] = x

    m = 0
    tau_tilde_storage = dict()
    for auth, attrs in attrs_by_auth.items():
        i = 0
        for attr in attrs:
            key = (auth, attr.lbl, attr)
            tau_tilde_storage[key] = i
            m = max(m, i)
            i += 1

    return m + 1


def compute_data():
    policy_len = 60
    divs = [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]
    data = dict()

    # vary_auth
    entry = {
        "div": [],
        "iota": [],
        "tau": [],
        "tau_tilde": [],
        "total": [],
        "total_2": []
    }
    for div in divs:
        input_gen = InputGen(policy_len, [div], 0, 0)
        for (_, uset) in input_gen.vary_auth():
            entry["div"].append(div)
            entry["iota"].append(iota(uset))
            entry["tau"].append(tau(uset))
            entry["tau_tilde"].append(tau_tilde(uset))
            entry["total"].append(div + tau(uset))
            entry["total_2"].append(div + div + tau(uset))
    data["auth"] = entry

    # auth_lbl
    entry = {
        "div": [],
        "iota": [],
        "tau": [],
        "tau_tilde": [],
        "total": [],
        "total_2": []
    }
    for div in divs:
        input_gen = InputGen(policy_len, [div], 0, 0)
        for (_, uset) in input_gen.vary_auth_lbl():
            entry["div"].append(div)
            entry["iota"].append(iota(uset))
            entry["tau"].append(tau(uset))
            entry["tau_tilde"].append(tau_tilde(uset))
            entry["total"].append(div + tau(uset))
            entry["total_2"].append(div + div + tau(uset))
    data["auth_lbl"] = entry

    # vary_auth_attr
    entry = {
        "div": [],
        "iota": [],
        "tau": [],
        "tau_tilde": [],
        "total": [],
        "total_2": []
    }
    for div in divs:
        input_gen = InputGen(policy_len, [div], 0, 0)
        for (_, uset) in input_gen.vary_auth_attr():
            entry["div"].append(div)
            entry["iota"].append(iota(uset))
            entry["tau"].append(tau(uset))
            entry["tau_tilde"].append(tau_tilde(uset))
            entry["total"].append(div + tau(uset))
            entry["total_2"].append(div + div + tau(uset))
    data["auth_attr"] = entry

    return data


def plot_auth(data):
    data = data["auth"]
    plt.style.use("ggplot")
    plt.rcParams.update({
        "text.usetex": True,
    })
    xs = [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]
    plt.plot(range(0, len(xs)), data["tau_tilde"], marker="s", label=r"$\tilde{\tau}$")
    plt.plot(range(0, len(xs)), data["iota"], marker="o", label=r"$\iota$, $\tau$")
    plt.xticks(range(0, len(xs)), labels=xs)
    plt.xlabel(r"$\| \tilde{\rho}(\Upsilon) \|$")
    plt.ylabel("\\# pairings")
    plt.title("auth")
    plt.legend()
    tikzplotlib.clean_figure()
    tikzplotlib.save("./auth_plot.tex")
    plt.close()


def plot_auth_lbl(data):
    data = data["auth_lbl"]
    plt.style.use("ggplot")
    plt.rcParams.update({
        "text.usetex": True,
    })
    xs = [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]
    plt.plot(range(0, len(xs)), data["tau_tilde"], marker="s", label=r"$\tilde{\tau}$, $\tau$, $\iota$")
    plt.xticks(range(0, len(xs)), labels=xs)
    plt.xlabel(r"$\| \tilde{\rho}(\Upsilon) \|$")
    plt.ylabel("\\# pairings")
    plt.title("auth/lbl")
    plt.legend()
    tikzplotlib.clean_figure()
    tikzplotlib.save("./auth_lbl_plot.tex")
    plt.close()


def plot_tradeoff(data):
    data = data["auth_attr"]
    plt.style.use("ggplot")
    plt.rcParams.update({
        "text.usetex": True,
    })
    xs = [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]
    plt.plot(range(0, len(xs)), data["tau_tilde"], marker="s", label=r"$\tilde{\tau}$")
    plt.plot(range(0, len(xs)), data["total"], marker="^", label=r"$\mathrm{total}$")
    opt_xs = []
    opt = []
    dir = "lt" if data["tau_tilde"][0] < data["total"][0] else "gt"
    for i in range(len(data["tau_tilde"])):
        opt_xs.append(i)
        x = data["tau_tilde"][i]
        y = data["total"][i]
        opt.append(min(x, y))
        if x < y and dir == "lt":
            pass
        elif x >= y and dir == "gt":
            pass
        else:
            # We have found the switching point
            dir = "lt" if data["tau_tilde"][i] < data["total"][i] else "gt"
            q_plus = max(data["tau_tilde"][i-1], data["total"][i-1])
            q_minus = min(data["tau_tilde"][i], data["total"][i])
            p_plus = max(data["tau_tilde"][i], data["total"][i])
            p_minus = min(data["tau_tilde"][i-1], data["total"][i-1])
            m = min(q_plus, q_minus, p_plus, p_minus)
            y = p_minus
            q_plus -= m
            q_minus -= m
            p_plus -= m
            p_minus -= m
            x_sec = q_plus / (p_plus + q_plus - p_minus - q_minus)
            opt_xs.insert(-1, i - 1 + x_sec)
            y_sec = y + x_sec*(p_plus - p_minus)
            opt.insert(-1, y_sec)
    min_plus = [m + 1 for m in opt]
    min_minus = [m - 1 for m in opt]
    plt.fill_between(opt_xs, min_minus, min_plus, color="yellow")
    plt.xticks(range(0, len(xs)), labels=xs)
    plt.xlabel(r"$\| \tilde{\rho}(\Upsilon) \|$")
    plt.ylabel("\\# pairings")
    plt.title("tradeoff")
    plt.legend()
    tikzplotlib.clean_figure()
    tikzplotlib.save("./tradeoff.tex")
    plt.close()



def main():
    data = compute_data()
    # print(data)
    plot_auth(data)
    plot_auth_lbl(data)
    plot_tradeoff(data)


main()