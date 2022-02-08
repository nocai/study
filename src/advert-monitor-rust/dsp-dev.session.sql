-- select * from plan where begin_time is null or end_time is null;
-- select * from dsp.plan where adconfig like '%os%'
-- select * from dsp.report where date = '2022-01-26' order by date DESC limit 1000;

select t.slot_id,
	t.date,
	sum(t.request) as req,
	sum(t.fill) as fil,
	sum(t.imp) as imp,
	sum(t.click) as cli
from report t
group by t.date,
	t.slot_id
having t.slot_id in (343436,300001)
	and t.date >= '2021-11-16'
	and t.date < '2022-01-27'
order by t.date desc