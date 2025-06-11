module.exports = {
  // Генерация случайного имени для команды
  generateTeamName: function() {
    return 'team_' + Math.random().toString(36).substring(7);
  },
  
  // Генерация случайного IP
  generateIP: function() {
    return '192.168.' + Math.floor(Math.random() * 255) + '.' + Math.floor(Math.random() * 255);
  },
  
  // Генерация случайного имени хоста
  generateHostname: function() {
    return 'host_' + Math.random().toString(36).substring(7) + '.com';
  }
};
