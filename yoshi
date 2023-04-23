#!/bin/bash


# Setting up string data
line1="............⠴⢿⣧⣤⣄........."
line2=".......⢀⣴⣿⣧⣆⣘⡄⢹⣿⣷⣆........"
line3="......⣴⣿⣿⣿⣿⣿⣿⣷⣾⣿⣿⣿⣷⡀......"
line4=".....⢀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣴⣿⣿⣿⣿⡀....."
line5="..⢀⣀⡀⣾⡿⠀⠉⠉⠛⠋⠛⠛⠚⣿⣿⣿⣿⣿⣿⣷⣄..."
line6=".⢠⣍⠹⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿⣿⣿⣿⣿⡿..."
line7=".⢿⣷⣾⣿⣿⠀⠀⠀⠀⠀⠀⢀⣴⣾⣿⣿⣿⣿⣿⣿⣷...."
line8=".⢹⣟⢻⣿⣄⠀⠀⠀⠀⣰⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇....."
line9=".⠻⠿⠟⠁⠑⢶⣤⣴⣿⣿⣿⣷⣶⣬⣿⣿⣿⡿......."
line10=".......⠈⠙⠛⠛⢛⣿⣿⣿⣿⡿⠛⠁......."
line11=".............⠻⢿⡿⠟........."
lines=($line1 $line2 $line3 $line4 $line5 $line6 $line7 $line8 $line9 $line10 $line11)

# TESTING: Showing that all the strings are the same length
: <<'END'
for line in "${lines[@]}"
do
    echo ${#line}
done
END

# Main loop
cycle=0
yoshiIndex=0 # yoshi width is 26 so go with 30 for loop
while [ true ] # change to not inturupted at some point
do

	# getting info about screen size
	windowDimensions=$(stty size)
	IFS=' ' read -ra windowYX <<< "$windowDimensions"
	windowLength=${windowYX[1]}
	yoshiScroll=$windowLength
	[ $windowLength -gt 40 ] && yoshiScroll=40
	
	outLines=()
	for line in "${lines[@]}"
	do
		outLine="${line:`expr $yoshiIndex % 26`:${#line}}${line:0:`expr $yoshiIndex % 26`}"
		outLines+=("${outLine}")
		
	done
	
	clear

	for outLine in "${outLines[@]}"
	do
		echo ${outLine:0:$yoshiScroll}
	done

	yoshiIndex=`expr $yoshiIndex + 1`
	sleep 0.05
	cycle=`expr $cycle + 1`
done


