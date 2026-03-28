vlans = [10, 20, 30, 40, 50]
blocked = 60

for vlan in vlans:
    print(f"VLAN {vlan}: Access to servers ALLOWED")
print(f"VLAN {blocked} (Guest): Access DENIED")