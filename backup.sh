#!/bin/bash

readonly SERVER="192.168.20.74"
readonly USER="root"

readonly MONTH="$(date +%m)"
readonly WEEK="$((($(date +%-d)-1)/7+1))"
readonly DAY="$(date -d "$D" '+%d')"
readonly HOUR="$(date +%H)"

readonly ARGS="-av --exclude={'docker-volumes/mariadb/ib_logfile0','docker-volumes/mariadb/ibtmp1'}"


backup_month() {
  ssh $USER@$SERVER "rsync $ARGS /mnt/user/docker-volumes /mnt/user/docker-backup/monthly/$MONTH/ && rm -rf /mnt/user/docker-backup/daily/* && rm -rf /mnt/user/docker-backup/hourly/*"
}

backup_week() {
  ssh $USER@$SERVER "rsync $ARGS /mnt/user/docker-volumes --compare-dest='/mnt/user/docker-backup/monthly/$MONTH/' /mnt/user/docker-backup/weekly/$WEEK/"
}

backup_day() {
  lastDay="$(date +%d -d "1 day ago")"
  ssh $USER@$SERVER "rsync $ARGS /mnt/user/docker-volumes --compare-dest='/mnt/user/docker-backup/monthly/$MONTH/' --compare-dest='/mnt/user/docker-backup/weekly/$WEEK/' --compare-dest=/mnt/user/docker-backup/daily/$lastDay/ /mnt/user/docker-backup/daily/$DAY/"
}

backup_hour() {
  lastDay="$(date +%d -d "1 day ago")"
  lastHour="$(date +%H -d "1 hour ago")"

  ssh $USER@$SERVER "rsync $ARGS /mnt/user/docker-volumes --compare-dest='/mnt/user/docker-backup/monthly/$MONTH/' --compare-dest='/mnt/user/docker-backup/weekly/$WEEK/' --compare-dest='/mnt/user/docker-backup/daily/$lastDay/' --compare-dest='/mnt/user/docker-backup/hourly/$lastHour/' /mnt/user/docker-backup/hourly/$HOUR/"
}
backup_$1