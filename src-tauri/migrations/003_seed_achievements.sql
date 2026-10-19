-- Гарантированный seed для базовых ачивок
INSERT OR IGNORE INTO achievements (
    code, category, icon, xp_reward, scope, tier, 
    condition_type, condition_value, title_key, description_key
) VALUES 
    ('first_step', 'progress', 'game-icons:footprint', 10, 'global', 'common', 
     'total_count', 1, 'achievements.ach.first_step.title', 'achievements.ach.first_step.desc');