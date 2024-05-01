#!/bin/sh
################################################################
# 	MacOS  : brew install dosfstools
# 	Ubuntu : apt-get install dosfstools
#	Usage:
# 		build_img.sh -m [arch] -fs [ext4|fat32] -file [testcast]
################################################################
# default setting
arch=x86_64
fs=fat32
FILE=

display_help()
{
	echo ""
	echo "./build_img.sh -m [arch] -fs [filesystem] -file [testcast]"
	echo ""
	exit 1
}

while [ -n "$1" ]; do
	case $1 in
		-m)
			shift
			arch="$1"
			;;
		-fs)
			shift
			fs="$1"
			;;
		-file)
			shift
			FILE="$1"
			;;
		riscv64)
			arch=riscv64
			;;
		x86_64)
			arch=x86_64
			;;
		aarch64)
			arch=aarch64
			;;
		fat32)
			fs=fat32
			;;
		ext4)
			fs=ext4
			;;
		riscv64_linux_musl)
			FILE=riscv64_linux_musl
			;;
		riscv64_gcc)
			FILE=riscv64_gcc
			;;
		riscv64_redis)
			FILE=riscv64_redis
			;;
		x86_64_linux_musl)
			FILE=x86_64_linux_musl
			;;
		x86_64_ZLM)
			FILE=x86_64_ZLM
			;;
		riscv64_libctest_dynamic)
			FILE=riscv64_libctest_dynamic
			;;
		riscv64_libctest_static)
			FILE=riscv64_libctest_static
			;;
		aarch64-linux-musl)
			FILE=aarch64-linux-musl
			;;
		*)
			display_help
			;;
	esac
	shift
done


if [ -z "$FILE" ]; then # use default testcases
	if [ "$arch" = "riscv64" ]; then
		FILE=riscv64_linux_musl
	elif [ "$arch" = "x86_64" ]; then
		FILE=x86_64_linux_musl
	elif [ "$arch" = "aarch64" ]; then
		FILE=aarch64-linux-musl
	else
		exit 1
	fi
fi

# 如果 testcases 下对应测例不存在，执行 submodules 拉取
if [ ! -d "./testcases/$FILE" ]; then
	git submodule update --init --recursive
fi

rm -f disk.img
dd if=/dev/zero of=disk.img bs=4M count=30

if [ "$fs" = "ext4" ]; then
	mkfs.ext4 -t ext4 disk.img
else
	fs=fat32
	mkfs.vfat -F 32 disk.img
fi

mkdir -p mnt

#### 添加 MacOS 支持
os=`uname -s`
if [ "x$os" = "xDarwin" ];then
	hdiutil detach mnt > /dev/null 2>&1
	hdiutil attach disk.img -mountpoint mnt
	echo "Copying $arch $fs $FILE/* to disk"
	cp -r ./testcases/$FILE/* ./mnt/
	hdiutil detach mnt
	chmod 777 disk.img
else
	sudo mount disk.img mnt
	# 根据命令行参数生成对应的测例
	echo "Copying $arch $fs $FILE/* to disk"
	sudo cp -r ./testcases/$FILE/* ./mnt/
	sudo umount mnt
	sudo rm -rf mnt
	sudo chmod 777 disk.img
fi