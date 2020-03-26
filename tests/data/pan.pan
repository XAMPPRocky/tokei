# 21 lines 11 code 4 comments 6 blanks

# Pan example code, see https://quattor-pan.readthedocs.io/en/stable/pan-book/index.html

prefix "/system/aii/osinstall/ks";
"clearpart" = append("vdb");
"ignoredisk" = list(); # no disks to ignore

prefix "/system/blockdevices";
"physical_devs/vdb/label" = "msdos";
"partitions/vdb1" = dict(
    "holding_dev", "vdb",
);

"files/{/srv/elasticsearch}" = dict('size', 0);

# To facilitate adding other partitions at a later stage, a
# logical volume will be created
"volume_groups/vg1/device_list" = append("partitions/vdb1");
"logical_volumes" = lvs_add('vg1', dict("elasticsearch", -1));

