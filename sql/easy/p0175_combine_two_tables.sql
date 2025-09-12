SELECT
	firstName AS firstName,
	lastName AS lastName,
	city AS city,
	state AS state
FROM
	person
LEFT JOIN address
	ON address.personId = person.personId;
