select *
from emp;

select empno,
       ename,
       job,
       mgr,
       hiredate,
       sal,
       comm,
       deptno
from emp;

select *
from emp
where deptno = 10;

select *
from emp
where deptno = 10
   or comm is not null
   or deptno = 20 and sal <= 2e3;

select ename, deptno, sal
from emp;

select sal salary, comm commission
from emp;

select *
from (select sal salary, comm commission from emp) _
where salary < 5e3;

select ename || ' WORKS AS A ' || job as occupation
from emp
where deptno = 10;

select ename,
       sal,
       case
         when sal <= 2e3 then 'UNDERPAID'
         when sal >= 4e3 then 'OVERPAID'
         else 'OK' end as status
from emp;

select *
from emp
limit 5;

select *
from emp
order by random()
limit 5;

select *
from emp
where comm is null;

select coalesce(comm, 0)
from emp;

select ename, job
from emp
where deptno in (10, 20)
  and (ename ilike '%I%' or job ilike '%ER');