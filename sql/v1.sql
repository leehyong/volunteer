create database if not exists volunteer ;
use volunteer;
CREATE TABLE if not exists `user` (
  `id` int NOT NULL AUTO_INCREMENT,
  `create_time` TIMESTAMP NOT NULL default current_timestamp,
  `update_time` TIMESTAMP NOT NULL default current_timestamp,
  `name` varchar(64) NOT NULL comment '名字',
  `parent_id` int default NULL comment '父账户id',
  `ancestor_id` int default NULL comment '祖先id',
  `depth`     tinyint NOT NULL comment '账号的层级深度',
  `mobile` varchar(20) NOT NULL comment '手机号',
  `lang` varchar(10) NOT NULL comment '语言',
  `country_code` char(4) NOT NULL comment '国家号',
  `role` varchar(10) NOT NULL comment '角色：admin，管理员；super，超管；manager，经理；normal，普通',
  `sex` char(1) NOT NULL comment '性别：男，女',
  `is_delete` tinyint not null default 0,
  PRIMARY KEY (`id`),
  KEY `ix_parent_id_depth_role` (`parent_id`, `depth`, `role`),
  KEY `ix_ancestor_id_role` (`ancestor_id`, `role`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 comment='用户表';

CREATE TABLE if not exists  `third_party_user` (
  `id` int NOT NULL AUTO_INCREMENT,
  `create_time` TIMESTAMP NOT NULL default current_timestamp,
  `update_time` TIMESTAMP NOT NULL default current_timestamp,
  `user_id` int NOT NULL comment '账户id',
  `account` varchar(64) NOT NULL comment '账户',
  `token` varchar(256) NOT NULL comment '第三方token',
  `type` varchar(16) NOT NULL comment '第三方登陆类型：wechat，微信；alipay，支付宝；taobao，淘宝；',
  `avatar` varchar(2048) NOT NULL default '' comment '第三方用户的头像链接',
  `is_delete` tinyint not null default 0,
  PRIMARY KEY (`id`),
  KEY `ix_account_type_is_delete` (`account`, `type`, `is_delete`),
  CONSTRAINT `third_party_user_fk1` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 comment='第三方用户登陆表';


CREATE TABLE if not exists `activity` (
  `id` int NOT NULL AUTO_INCREMENT,
  `create_time` TIMESTAMP NOT NULL default current_timestamp,
  `update_time` TIMESTAMP NOT NULL default current_timestamp,
  `start_time`  TIMESTAMP NOT NULL comment '活动开始时间',
  `end_time`  TIMESTAMP NOT NULL comment '活动结束时间',
  `creator_id` int NOT NULL comment '创建者id',
  `last_editor_id` int NOT NULL comment '上次编辑者id',
  `subject` varchar(512) NOT NULL comment '主题',
  `type`  char(16) NOT NULL comment '类型:gym, 体育；meeting， 会议；',
  `apply_url`  varchar(2048) NOT NULL comment '报名链接',
  `text`  text NOT NULL comment '内容',
  `is_delete` tinyint not NULL default 0 comment '是否删除',
  PRIMARY KEY (`id`),
  KEY `ix_start_time_end_time_type` (`start_time`, `end_time`, `type`),
  CONSTRAINT `avtivity_fk1` FOREIGN KEY (`last_editor_id`) REFERENCES `user` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 comment='志愿者活动表';


CREATE TABLE if not exists  `apply` (
  `id` int NOT NULL AUTO_INCREMENT,
  `create_time` TIMESTAMP NOT NULL default current_timestamp,
  `update_time` TIMESTAMP NOT NULL default current_timestamp,
  `user_id` int NOT NULL comment '创建者id',
  `activity_id` int NOT NULL comment '创建者id',
  `is_delete` tinyint not NULL DEFAULT 0 comment '是否删除',
  PRIMARY KEY (`id`),
  CONSTRAINT `apply_fk1` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`),
  CONSTRAINT `apply_fk2` FOREIGN KEY (`activity_id`) REFERENCES `activity` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 comment='志愿者报名表';

