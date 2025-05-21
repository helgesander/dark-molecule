# Отчет по проекту: {{project.name}}

**ID проекта:** {{project.id}}
**Дата начала:** {{formatDate project.start_date}}
**Дата окончания:** {{formatDate project.end_date}}
**Область исследования:** {{project.scope}}

## Описание проекта
{{project.description}}

## Результаты исследования

### Обнаруженные хосты
{{#each project.hosts}}
#### {{this.hostname}}
- **IP адрес:** {{this.ip_address}}
- **Статус:** {{this.status}}
- **Операционная система:** {{this.os}}
- **Открытые порты:** {{this.open_ports}}
{{/each}}

### Найденные уязвимости
{{#each project.issues}}
#### {{this.title}}
- **Уровень риска:** {{this.severity}}
- **Статус:** {{this.status}}
- **Описание:** {{this.description}}
- **Рекомендации по исправлению:** {{this.recommendation}}
{{/each}}

### Статистика
- **Всего хостов:** {{project.hosts.length}}
- **Всего уязвимостей:** {{project.issues.length}}
- **Критических:** {{countIssuesBySeverity project.issues "critical"}}
- **Высоких:** {{countIssuesBySeverity project.issues "high"}}
- **Средних:** {{countIssuesBySeverity project.issues "medium"}}
- **Низких:** {{countIssuesBySeverity project.issues "low"}}

## Рекомендации по безопасности
{{#each project.issues}}
### {{this.title}}
{{this.recommendation}}
{{/each}}

## Приложения

### A. Детали хостов
{{#each project.hosts}}
#### {{this.hostname}}
- **IP:** {{this.ip_address}}
- **OS:** {{this.os}}
- **Статус:** {{this.status}}
- **Порты:** {{this.open_ports}}
{{/each}}

### B. Детали уязвимостей
{{#each project.issues}}
#### {{this.title}}
- **Уровень риска:** {{this.severity}}
- **Статус:** {{this.status}}
- **Описание:** {{this.description}}
- **Рекомендации:** {{this.recommendation}}
{{/each}}

---
*Отчет сгенерирован автоматически системой Dark Molecule*
*Дата генерации: {{formatDate now}}* 