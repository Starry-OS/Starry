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
	# 若不指定参数，则使用默认的测例
	echo "  -m | --arch		architecture: x86_64|riscv64|aarch64", default is x86_64
	echo "  -fs | --filesystem	filesystem: ext4|fat32", default is fat32
	echo "  -file | --testcase  If not specified, use the default testcases for different architectures."
	echo "  default testcases:"
	echo "    x86_64: x86_64_linux_musl"
	echo "    riscv64: riscv64_linux_musl"
	echo "    aarch64: aarch64-linux-musl"
	echo "  -h | --help		display help"
	echo ""
	exit 1
}

# 可能接受三类参数 -m [arch] -fs [filesystem] -file [testcast]
# 但是不一定只有一个参数，所以使用 while 循环
while [ "$1" != "" ]; do
	case $1 in
		-m | --arch )	shift
						arch=$1
						;;
		-fs | --filesystem )	shift
						fs=$1
						;;
		-file | --testcase )	shift
						FILE=$1
						;;
		-h | --help )		display_help
						exit
						;;
		* )					display_help
						exit 1
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
		echo "Unknown architecture: $arch"
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
else if [ "$fs" = "fat32" ]; then
	fs=fat32
	mkfs.vfat -F 32 disk.img
else
	echo "Unknown filesystem: $fs"
	exit 1
fi
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