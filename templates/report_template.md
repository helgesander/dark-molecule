<div style="font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; color: #333;">
    <div style="text-align: center; margin-bottom: 30px;">
        <svg width="100" height="100" viewBox="0 0 100 100" style="margin-bottom: 20px;">
            <circle cx="50" cy="50" r="45" fill="#2c3e50"/>
            <path d="M30 50 L45 65 L70 35" stroke="white" stroke-width="5" fill="none"/>
        </svg>
        <h1 style="color: #2c3e50; font-size: 28px; margin: 0;">Отчет по проекту: {{project.name}}</h1>
    </div>

    <div style="background: #f8f9fa; padding: 20px; border-radius: 8px; margin-bottom: 30px;">
        <p style="margin: 5px 0;"><strong style="color: #2c3e50;">ID проекта:</strong> {{project.id}}</p>
        <p style="margin: 5px 0;"><strong style="color: #2c3e50;">Дата начала:</strong> {{formatDate project.start_date}}</p>
        <p style="margin: 5px 0;"><strong style="color: #2c3e50;">Дата окончания:</strong> {{formatDate project.end_date}}</p>
        <p style="margin: 5px 0;"><strong style="color: #2c3e50;">Область исследования:</strong> {{project.scope}}</p>
    </div>

    <div style="margin-bottom: 30px;">
        <h2 style="color: #2c3e50; border-bottom: 2px solid #3498db; padding-bottom: 10px;">Описание проекта</h2>
        <p style="line-height: 1.6;">{{project.description}}</p>
    </div>

    <div style="margin-bottom: 30px;">
        <h2 style="color: #2c3e50; border-bottom: 2px solid #3498db; padding-bottom: 10px;">Результаты исследования</h2>

        <h3 style="color: #34495e;">Обнаруженные хосты</h3>
        {{#each project.hosts}}
        <div style="background: #f8f9fa; padding: 15px; border-radius: 8px; margin-bottom: 15px;">
            <h4 style="color: #2c3e50; margin-top: 0;">{{this.hostname}}</h4>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">IP адрес:</strong> {{this.ip_address}}</p>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">Статус:</strong> {{this.status}}</p>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">Операционная система:</strong> {{this.os}}</p>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">Открытые порты:</strong> {{this.open_ports}}</p>
        </div>
        {{/each}}

        <h3 style="color: #34495e;">Найденные уязвимости</h3>
        {{#each project.issues}}
        <div style="background: #f8f9fa; padding: 15px; border-radius: 8px; margin-bottom: 15px; border-left: 4px solid {{#if (eq this.severity "critical")}}#e74c3c{{else if (eq this.severity "high")}}#e67e22{{else if (eq this.severity "medium")}}#f1c40f{{else}}#2ecc71{{/if}};">
            <h4 style="color: #2c3e50; margin-top: 0;">{{this.title}}</h4>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">Уровень риска:</strong> {{this.severity}}</p>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">Статус:</strong> {{this.status}}</p>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">Описание:</strong> {{this.description}}</p>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">Рекомендации по исправлению:</strong> {{this.recommendation}}</p>
        </div>
        {{/each}}

        <div style="background: #2c3e50; color: white; padding: 20px; border-radius: 8px; margin-top: 30px;">
            <h3 style="margin-top: 0; color: white;">Статистика</h3>
            <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 15px;">
                <div style="text-align: center;">
                    <p style="margin: 5px 0; font-size: 24px;">{{array_length project.hosts}}</p>
                    <p style="margin: 5px 0; color: #bdc3c7;">Всего хостов</p>
                </div>
                <div style="text-align: center;">
                    <p style="margin: 5px 0; font-size: 24px;">{{array_length project.issues}}</p>
                    <p style="margin: 5px 0; color: #bdc3c7;">Всего уязвимостей</p>
                </div>
                <div style="text-align: center;">
                    <p style="margin: 5px 0; font-size: 24px;">{{countIssuesBySeverity project.issues "critical"}}</p>
                    <p style="margin: 5px 0; color: #bdc3c7;">Критических</p>
                </div>
                <div style="text-align: center;">
                    <p style="margin: 5px 0; font-size: 24px;">{{countIssuesBySeverity project.issues "high"}}</p>
                    <p style="margin: 5px 0; color: #bdc3c7;">Высоких</p>
                </div>
                <div style="text-align: center;">
                    <p style="margin: 5px 0; font-size: 24px;">{{countIssuesBySeverity project.issues "medium"}}</p>
                    <p style="margin: 5px 0; color: #bdc3c7;">Средних</p>
                </div>
                <div style="text-align: center;">
                    <p style="margin: 5px 0; font-size: 24px;">{{countIssuesBySeverity project.issues "low"}}</p>
                    <p style="margin: 5px 0; color: #bdc3c7;">Низких</p>
                </div>
            </div>
        </div>
    </div>

    <div style="margin-bottom: 30px;">
        <h2 style="color: #2c3e50; border-bottom: 2px solid #3498db; padding-bottom: 10px;">Рекомендации по безопасности</h2>
        {{#each project.issues}}
        <div style="background: #f8f9fa; padding: 15px; border-radius: 8px; margin-bottom: 15px;">
            <h3 style="color: #2c3e50; margin-top: 0;">{{this.title}}</h3>
            <p style="line-height: 1.6;">{{this.recommendation}}</p>
        </div>
        {{/each}}
    </div>

    <div style="margin-bottom: 30px;">
        <h2 style="color: #2c3e50; border-bottom: 2px solid #3498db; padding-bottom: 10px;">Приложения</h2>

        <h3 style="color: #34495e;">A. Детали хостов</h3>
        {{#each project.hosts}}
        <div style="background: #f8f9fa; padding: 15px; border-radius: 8px; margin-bottom: 15px;">
            <h4 style="color: #2c3e50; margin-top: 0;">{{this.hostname}}</h4>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">IP:</strong> {{this.ip_address}}</p>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">OS:</strong> {{this.os}}</p>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">Статус:</strong> {{this.status}}</p>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">Порты:</strong> {{this.open_ports}}</p>
        </div>
        {{/each}}

        <h3 style="color: #34495e;">B. Детали уязвимостей</h3>
        {{#each project.issues}}
        <div style="background: #f8f9fa; padding: 15px; border-radius: 8px; margin-bottom: 15px; border-left: 4px solid {{#if (eq this.severity "critical")}}#e74c3c{{else if (eq this.severity "high")}}#e67e22{{else if (eq this.severity "medium")}}#f1c40f{{else}}#2ecc71{{/if}};">
            <h4 style="color: #2c3e50; margin-top: 0;">{{this.title}}</h4>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">Уровень риска:</strong> {{this.severity}}</p>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">Статус:</strong> {{this.status}}</p>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">Описание:</strong> {{this.description}}</p>
            <p style="margin: 5px 0;"><strong style="color: #3498db;">Рекомендации:</strong> {{this.recommendation}}</p>
        </div>
        {{/each}}
    </div>

    <div style="text-align: center; color: #7f8c8d; font-size: 12px; margin-top: 50px; padding-top: 20px; border-top: 1px solid #ecf0f1;">
        <p style="margin: 5px 0;">Отчет сгенерирован автоматически системой Dark Molecule</p>
        <p style="margin: 5px 0;">Дата генерации: {{formatDate now}}</p>
    </div>
</div> 