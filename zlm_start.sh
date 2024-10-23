make clean
./build_img.sh -a x86_64 -fs ext4  -s 80 -file x86_64_ZLM
make A=apps/monolithic_userboot FEATURES=another_ext4,img,sched_rr LOG=error NET=y BLK=y ARCH=x86_64 ACCEL=n run
