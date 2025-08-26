# -*- coding: utf-8 -*-
"""
Created on Thu Jun 20 11:13:44 2024

@author: Marloes
"""

import numpy as np

# this function creates an AND-policy matrix of certain length
def make_policy_matrix(length):
    ones = [1 for i in range(length)]
    policy_matrix = [ones]
    for i in range(1,length):
        new_row = [0 for i in range(length)]
        new_row[i] = -1
        policy_matrix.append(new_row)
    return policy_matrix

# we start by defining some test policies
policy_length1 = 10
policy_matrix1 = make_policy_matrix(policy_length1)
rho1 = ['att' + str(i) for i in range(policy_length1)]
rho_lab1 = ['lab' + str(i) for i in range(policy_length1)]
rho_tilde1 = ['aut1' for i in range(int(np.ceil(policy_length1/2)))] + ['aut2' for i in range(int(np.ceil(policy_length1/2)))]

# example with more duplicates
policy_length2 = 10
policy_matrix2 = policy_matrix1
rho2 = rho1[:5] + rho1[:5]
rho_lab2 = ['lab1' for i in range(3)] + ['lab2' for i in range(3)] + ['lab3' for i in range(4)]
rho_tilde2 = ['aut1' for i in range(int(np.ceil(policy_length2/2)))] + ['aut2' for i in range(int(np.ceil(policy_length2/2)))]

# define the deduplication in relation to the policy mappings as follows:
# the domain (i.e., input space) of the deduplication mapping: input_domain
# the subdomain on which the deduplication mapping needs to be injective: subdomain_injective

# the first example is for injectivity restriction on the attributes
# i.e., randomizers can be reused among attributes

# the second example is for injectivity restriction on the authorities
# i.e., randomizers can be reused among authorities

# the third example is for injectivity restriction on the authority-label pairs
# i.e., each row j associated with the same pair (authority, label) needs to be
# mapped to a unique value

# for all examples, the input domain is [n_1], i.e.,
input_domain1 = [i + 1 for i in range(policy_length1)]
input_domain2 = [i + 1 for i in range(policy_length2)]

subdomain_injective_ex1 = ['rho']
subdomain_injective_ex2 = ['rho_tilde']
subdomain_injective_ex3 = ['rho_tilde', 'rho_lab']

# expected output for the first and third test policy (resp) and example 1
expected_dedup_map1_ex1 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
expected_dedup_map1_ex2 = [1, 2, 3, 4, 5, 1, 2, 3, 4, 5]
expected_dedup_map1_ex3 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
expected_dedup_map2_ex1 = [1, 1, 1, 1, 1, 2, 2, 2, 2, 2]
expected_dedup_map2_ex2 = [1, 2, 3, 4, 5, 1, 2, 3, 4, 5]
expected_dedup_map2_ex3 = [1, 2, 3, 1, 2, 1, 1, 2, 3, 4]

# extra inputs are of the form rho_lab, rho_tilde, rho_neg
def create_dedup_mapping_policy(policy_length, rho, input_domain, subdomain_injective, **kwargs):
    key_value_pairs = [('rho', rho)]
    if 'rho' in subdomain_injective:
        subdomain_injective_val = [('rho', rho)]
    else:
        subdomain_injective_val = []
    for key, value in kwargs.items():
        key_value_pairs.append((key, value))
        if str(key) in subdomain_injective:
            subdomain_injective_val.append((str(key), value))
    
    if len(subdomain_injective_val) != len(subdomain_injective):
        return 'Injectivity subdomain contains inputs that were not specified.'
        
    # create lists with all the possible different outputs
    subdomain_diff_vals = []
    for (key, sub_val) in subdomain_injective_val:
        lis_vals = []
        for val in sub_val:
            if not val in lis_vals:
                lis_vals.append(val)
        subdomain_diff_vals.append((key, sub_val, lis_vals))
        
    # make all possible different combinations that occur in the subdomain
    # for which an injectivity restriction holds
    lis_diff_i_s_combis = []
    for (_, _, lis_vals) in subdomain_diff_vals:
        if len(lis_diff_i_s_combis) == 0:
            lis_diff_i_s_combis = [[i] for i in lis_vals]
        else:
            lis_diff_i_s_combis_new = []
            for entry in lis_vals:
                for lis in lis_diff_i_s_combis:
                    lis_diff_i_s_combis_new.append(lis + [entry])
            lis_diff_i_s_combis = lis_diff_i_s_combis_new
            
    # print("Codomain")
    # print(lis_diff_i_s_combis)
    
    # for each combination of values in the subdomain with an injectivity restriction, 
    # determine the inputs in the domain that are mapped to it
    lis_diff_i_s_combis_domain = []
    for lis in lis_diff_i_s_combis:
        sublist_inputs = []
        for i, inp in enumerate(input_domain):
            input_mapped_to_tuple = True
            for j, (key, lis_vals) in enumerate(subdomain_injective_val):
                if lis_vals[i] != lis[j]:
                    input_mapped_to_tuple = False
            if input_mapped_to_tuple:
                sublist_inputs.append(inp)
        lis_diff_i_s_combis_domain.append(sublist_inputs)
        
    # print("Domains mapped to codomain")
    # print(lis_diff_i_s_combis_domain)
    
    # assign outputs to each list in the subdomain with injectivity restriction -> domain map
    # i.e., for each tuple for which we consider the inputs that are mapped to it
    # we map each input to a different integer
    lis_diff_i_s_combis_domain_dedup = []
    max_val_codomain = 0
    for lis in lis_diff_i_s_combis_domain:
        lis_diff_i_s_combis_domain_dedup.append([i + 1 for i in range(len(lis))])
        if len(lis) > max_val_codomain:
            max_val_codomain = len(lis)
    
    # print("Co-domain of the dedup map")
    # print(lis_diff_i_s_combis_domain_dedup)
    
    # find for each input in the domain the integer in the co-domain based on 
    # the mappings provided by lis_diff_i_s_combis_domain_dedup
    dedup_mapping = [0 for i in range(len(input_domain))]
    for i, inp in enumerate(input_domain):
        for j, lis in enumerate(lis_diff_i_s_combis_domain):
            for k, val in enumerate(lis):
                if val == inp:
                    dedup_mapping[i] += lis_diff_i_s_combis_domain_dedup[j][k]
    
    # print("Dedup mapping")
    print(dedup_mapping)
    return dedup_mapping

print(create_dedup_mapping_policy(policy_length1, rho1, input_domain1, subdomain_injective_ex1, rho_lab = rho_lab1, rho_tilde = rho_tilde1) == expected_dedup_map1_ex1)
print(create_dedup_mapping_policy(policy_length1, rho1, input_domain1, subdomain_injective_ex2, rho_lab = rho_lab1, rho_tilde = rho_tilde1) == expected_dedup_map1_ex2)
print(create_dedup_mapping_policy(policy_length1, rho1, input_domain1, subdomain_injective_ex3, rho_lab = rho_lab1, rho_tilde = rho_tilde1) == expected_dedup_map1_ex3)

print(create_dedup_mapping_policy(policy_length2, rho2, input_domain2, subdomain_injective_ex1, rho_lab = rho_lab2, rho_tilde = rho_tilde2) == expected_dedup_map2_ex1)
print(create_dedup_mapping_policy(policy_length2, rho2, input_domain2, subdomain_injective_ex2, rho_lab = rho_lab2, rho_tilde = rho_tilde2) == expected_dedup_map2_ex2)
print(create_dedup_mapping_policy(policy_length2, rho2, input_domain2, subdomain_injective_ex3, rho_lab = rho_lab2, rho_tilde = rho_tilde2) == expected_dedup_map2_ex3)

# extra inputs are of the form rho_lab, rho_tilde, rho_neg
def alternative_create_dedup_mapping_policy(policy_length, rho, input_domain, subdomain_injective, **kwargs):
    key_value_pairs = [('rho', rho)]
    domain_val = [('rho', rho)]
    if 'rho' in subdomain_injective:
        subdomain_injective_val = [('rho', rho)]
        special_indices = [0]
    else:
        subdomain_injective_val = []
        special_indices = []
    ctr = 1
    for key, value in kwargs.items():
        key_value_pairs.append((key, value))
        domain_val.append((str(key), value))
        if str(key) in subdomain_injective:
            subdomain_injective_val.append((str(key), value))
            special_indices.append(ctr)
        ctr += 1
    
    if len(subdomain_injective_val) != len(subdomain_injective):
        return 'Injectivity subdomain contains inputs that were not specified.'
    
    # from 'merged_list' till 'remove_dups' can be done more efficiently
    # we don't need to first create domain and then cut out the injectivity subdomains
    # we can directly work with that
    merged_list = [[0 for j in range(len(domain_val))] for i in range(policy_length)]
    for j, (key, list_vals) in enumerate(domain_val):
        for i, val in enumerate(list_vals):
            merged_list[i][j] = val
    
    remove_dups_from_merge_list = []
    for lis in merged_list:
        new_lis = []
        for ind in special_indices:
            new_lis.append(lis[ind])
        remove_dups_from_merge_list.append(new_lis)
    
    lis_dups = []    
    lis_ctrs = []
    dedup_mapping = [0 for i in range(len(input_domain))]
    for i, tup in enumerate(remove_dups_from_merge_list):
        if not tup in lis_dups:
            lis_dups.append(tup)
            lis_ctrs.append(1)
            dedup_val = 1
        else:
            ind = lis_dups.index(tup)
            lis_ctrs[ind] += 1
            dedup_val = lis_ctrs[ind]
        dedup_mapping[i] = dedup_val
            
    print(dedup_mapping)
    
    return(dedup_mapping)

print(alternative_create_dedup_mapping_policy(policy_length1, rho1, input_domain1, subdomain_injective_ex1, rho_lab = rho_lab1, rho_tilde = rho_tilde1) == expected_dedup_map1_ex1)
print(alternative_create_dedup_mapping_policy(policy_length1, rho1, input_domain1, subdomain_injective_ex2, rho_lab = rho_lab1, rho_tilde = rho_tilde1) == expected_dedup_map1_ex2)
print(alternative_create_dedup_mapping_policy(policy_length1, rho1, input_domain1, subdomain_injective_ex3, rho_lab = rho_lab1, rho_tilde = rho_tilde1) == expected_dedup_map1_ex3)

print(alternative_create_dedup_mapping_policy(policy_length2, rho2, input_domain2, subdomain_injective_ex1, rho_lab = rho_lab2, rho_tilde = rho_tilde2) == expected_dedup_map2_ex1)
print(alternative_create_dedup_mapping_policy(policy_length2, rho2, input_domain2, subdomain_injective_ex2, rho_lab = rho_lab2, rho_tilde = rho_tilde2) == expected_dedup_map2_ex2)
print(alternative_create_dedup_mapping_policy(policy_length2, rho2, input_domain2, subdomain_injective_ex3, rho_lab = rho_lab2, rho_tilde = rho_tilde2) == expected_dedup_map2_ex3)

# extra inputs are of the form rho_lab, rho_tilde, rho_neg
def alternative2_create_dedup_mapping_policy(policy_length, rho, input_domain, subdomain_injective, **kwargs):
    key_value_pairs = [('rho', rho)]
    if 'rho' in subdomain_injective:
        subdomain_injective_val = [('rho', rho)]
        special_indices = [0]
    else:
        subdomain_injective_val = []
    ctr = 1
    for key, value in kwargs.items():
        key_value_pairs.append((key, value))
        if str(key) in subdomain_injective:
            subdomain_injective_val.append((str(key), value))
        ctr += 1
    
    if len(subdomain_injective_val) != len(subdomain_injective):
        return 'Injectivity subdomain contains inputs that were not specified.'
    
    merged_list_subdomain = [[0 for j in range(len(subdomain_injective_val))] for i in range(policy_length)]
    for j, (key, list_vals) in enumerate(subdomain_injective_val):
        for i, val in enumerate(list_vals):
            merged_list_subdomain[i][j] = val
    
    lis_dups = []    
    lis_ctrs = []
    dedup_mapping = [0 for i in range(len(input_domain))]
    for i, tup in enumerate(merged_list_subdomain):
        if not tup in lis_dups:
            lis_dups.append(tup)
            lis_ctrs.append(1)
            dedup_val = 1
        else:
            ind = lis_dups.index(tup)
            lis_ctrs[ind] += 1
            dedup_val = lis_ctrs[ind]
        dedup_mapping[i] = dedup_val
            
    print(dedup_mapping)
    
    return(dedup_mapping)

print(alternative2_create_dedup_mapping_policy(policy_length1, rho1, input_domain1, subdomain_injective_ex1, rho_lab = rho_lab1, rho_tilde = rho_tilde1) == expected_dedup_map1_ex1)
print(alternative2_create_dedup_mapping_policy(policy_length1, rho1, input_domain1, subdomain_injective_ex2, rho_lab = rho_lab1, rho_tilde = rho_tilde1) == expected_dedup_map1_ex2)
print(alternative2_create_dedup_mapping_policy(policy_length1, rho1, input_domain1, subdomain_injective_ex3, rho_lab = rho_lab1, rho_tilde = rho_tilde1) == expected_dedup_map1_ex3)

print(alternative2_create_dedup_mapping_policy(policy_length2, rho2, input_domain2, subdomain_injective_ex1, rho_lab = rho_lab2, rho_tilde = rho_tilde2) == expected_dedup_map2_ex1)
print(alternative2_create_dedup_mapping_policy(policy_length2, rho2, input_domain2, subdomain_injective_ex2, rho_lab = rho_lab2, rho_tilde = rho_tilde2) == expected_dedup_map2_ex2)
print(alternative2_create_dedup_mapping_policy(policy_length2, rho2, input_domain2, subdomain_injective_ex3, rho_lab = rho_lab2, rho_tilde = rho_tilde2) == expected_dedup_map2_ex3)