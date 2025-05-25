# Отчет по проекту {{project.name}}

## Общая информация
- **Дата создания отчета:** {{formatDate now}}
- **Дата создания проекта:** {{formatDate project.created_at}}
- **Статус проекта:** {{project.status}}

## Статистика по уязвимостям
- **Всего уязвимостей:** {{project.issueslength}}
- **Критические:** {{~countIssuesBySeverity project.issues "critical"~}}
- **Высокие:** {{~countIssuesBySeverity project.issues "high"~}}
- **Средние:** {{~countIssuesBySeverity project.issues "medium"~}}
- **Низкие:** {{~countIssuesBySeverity project.issues "low"~}}

## Список уязвимостей

{{#each project.issues}}
### {{title}}
- **Уровень риска:** {{severity}}
- **Описание:** {{description}}
- **Рекомендации:** {{recommendations}}
{{/each}}

## Заключение
Проект {{project.name}} содержит {{project.issues.length}} уязвимостей различного уровня риска. 
Рекомендуется обратить особое внимание на критические и высокие уязвимости. 