#!/usr/bin/env python3

import random

class And:
    def __init__(self, lhs, rhs):
        self.lhs = lhs
        self.rhs = rhs

    def __format__(self, fmt_spec):
        return f"({self.lhs} & {self.rhs})"

    def eval(self, keys):
        return self.lhs.eval(keys) and self.rhs.eval(keys)


class Or:
    def __init__(self, lhs, rhs):
        self.lhs = lhs
        self.rhs = rhs

    def __format__(self, fmt_spec):
        return f"({self.lhs} | {self.rhs})"

    def eval(self, keys):
        return self.lhs.eval(keys) or self.rhs.eval(keys)


class Term:
    def __init__(self, auth, lbl, attr, neg=False, idx=None):
        self.auth = auth
        self.lbl = lbl
        self.attr = attr
        self.neg = neg
        self.idx = idx

    def __format__(self, fmt_spec):
        return f"{'!' if self.neg else ''}{self.auth}.{self.lbl}:{self.attr}{f'_{self.idx:02}' if self.idx is not None else ''}"
    
    def __repr__(self):
        return f"{self}"

    def eval(self, keys):
        if self.neg:
            ks = [k for k in keys if k.auth == self.auth and k.lbl == self.lbl]
            return len(ks) > 0 and all(k.attr != self.attr or k.idx != self.idx for k in ks)
        else:
            return any(self.auth == k.auth and self.lbl == k.lbl and self.attr == k.attr for k in keys)


class PolicyGenerator:
    authorities = "ABCD"
    labels = "abcde"
    attributes = "0123456"

    def rand_auth(self):
        return random.choice(self.authorities)

    def rand_lbl(self):
        return random.choice(self.labels)

    def rand_attr(self):
        return random.choice(self.attributes)

    def rand_neg(self):
        weights = [1, 4]
        return random.choices([True, False], weights=weights)[0]

    def rand_term(self):
        t = Term(self.rand_auth(), self.rand_lbl(), self.rand_attr(), neg=self.rand_neg())
        if t.neg:
            degree = random.randint(1, 4)
            return t, [Term(t.auth, t.lbl, t.attr, idx=d) for d in range(degree)]
        else:
            return t, [t]

    def split(self, x):
        r = random.randint(0, x)
        return r, x - r

    def generate(self, fuel, max_attempts=1_000):
        # There are some corner-cases when this fails, e.g., 
        # if a negated attributes is used without negation elsewhere
        for _ in range(max_attempts):
            policy, ok_set = self._generate_ok(fuel)
            if policy.eval(ok_set):
                break
        fail_set = self._generate_fail(policy, ok_set, max_attempts=max_attempts)
        return policy, ok_set, fail_set

    def _generate_ok(self, fuel):
        if fuel <= 1:
            return self.rand_term()
        else:
            f = random.random()
            if f < 0.20:
                return self.rand_term()
            elif f < 0.60:
                l, r = self.split(fuel)
                lhs, ls = self._generate_ok(l)
                rhs, rs = self._generate_ok(r)
                return And(lhs, rhs), ls + rs
            else:
                l, r = self.split(fuel)
                lhs, ls = self._generate_ok(l)
                rhs, rs = self._generate_ok(r)
                if random.random() < 0.50:
                    return Or(lhs, rhs), ls
                else:
                    return Or(lhs, rhs), rs

    def _generate_fail(self, policy, ok_set, max_attempts=1_000):
        f = random.random()
        fail_set = ok_set
        neg_set = [t for t in fail_set if t.neg]
        for i in range(max_attempts):
            if f < 0.05:
                # Empty user set
                fail_set = []
            elif f < 0.50 and neg_set:
                # Missing alternatives for negation
                res = []
                done = False
                for t in fail_set:
                    if t.idx and not done:
                        done = True
                    else:
                        res += t
                fail_set = res
            elif f < 0.75 and neg_set:
                # Contradiction
                t = random.shuffle(neg_set)[0]
                fail_set += [Term(t.auth, t.lbl, t.attr)]
            else:
                # Missing attribute(s)
                fail_set = random.sample(fail_set, k=len(fail_set) - 1)

            if not policy.eval(fail_set):
                return fail_set
        raise RuntimeError("Could not find a failing set for policy")


def print_test_case(policy, user_attrs, idx, is_ok):
    user_attrs = ", ".join(f"\"{ua}\"" for ua in user_attrs)
    s = f"""
#[test]
fn opt0_generated_test_case_{idx:03}_{"ok" if is_ok else "fail"}() {{
   let user_attrs = vec![{user_attrs}];
   let policy = "{policy}";
   assert_decryption_{"ok" if is_ok else "fail"}(user_attrs, policy);
}}
"""
    print(s)


def main():
    generator = PolicyGenerator()
    for i in range(100):
        policy, ok_set, fail_set = generator.generate(5)
        print_test_case(policy, ok_set, i, is_ok=True)
        assert policy.eval(ok_set), "Ok set is wrong"
        print_test_case(policy, fail_set, i, is_ok=False)
        assert not policy.eval(fail_set), "Fail set is wrong"


if __name__ == "__main__":
    main()
