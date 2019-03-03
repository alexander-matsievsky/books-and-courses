create table emp
(
  empno    int  not null,
  ename    text not null,
  job      text not null,
  mgr      int,
  hiredate date not null,
  sal      int  not null,
  comm     int,
  deptno   int  not null
);

create table dept
(
  deptno int  not null,
  dname  text not null,
  loc    text not null
);

create table t1
(
  id int not null
);

create table t10
(
  id int not null
);

create table t100
(
  id int not null
);

create table t500
(
  id int not null
);

copy emp (empno, ename, job, mgr, hiredate, sal, comm, deptno)
  from '/tmp/import/emp.csv' delimiter ',' csv header;

copy dept (deptno, dname, loc)
  from '/tmp/import/dept.csv' delimiter ',' csv header;

insert into t1 (id)
select i
from generate_series(1, 1) as s(i);

insert into t10 (id)
select i
from generate_series(1, 10) as s(i);

insert into t100 (id)
select i
from generate_series(1, 100) as s(i);

insert into t500 (id)
select i
from generate_series(1, 500) as s(i);