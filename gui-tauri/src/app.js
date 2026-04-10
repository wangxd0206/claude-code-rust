// Claude Code - Frontend Application
//
// This script implements the complete GUI functionality
// to replicate the original Claude Code experience.

import { invoke } from '@tauri-apps/api/core';

let chatHistory = [];
let settings = {
  apiKey: '',
  baseUrl: 'https://api.anthropic.com',
  model: 'claude-3-opus-20240229',
  theme: 'dark',
  autoSave: true,
  maxTokens: 4096,
  temperature: 0.7
};
let currentDirectory = '';

// Initialize application
document.addEventListener('DOMContentLoaded', () => {
  initializeApp();
  setupEventListeners();
  loadSettings();
  loadCurrentDirectory();
});

function initializeApp() {
  const welcomeMessage = document.querySelector('.welcome-message');
  if (welcomeMessage) {
    welcomeMessage.style.opacity = '1';
  }
}

function setupEventListeners() {
  // Navigation
  document.querySelectorAll('.nav-btn').forEach(btn => {
    btn.addEventListener('click', () => switchPanel(btn.dataset.tab));
  });

  // Chat input
  const chatInput = document.getElementById('chatInput');
  const sendBtn = document.getElementById('sendBtn');

  if (chatInput && sendBtn) {
    chatInput.addEventListener('keydown', handleChatKeydown);
    sendBtn.addEventListener('click', sendMessage);
  }

  // Quick actions
  document.querySelectorAll('.quick-action').forEach(btn => {
    btn.addEventListener('click', () => {
      const prompt = btn.dataset.prompt;
      if (prompt && chatInput) {
        chatInput.value = prompt;
        sendMessage();
      }
    });
  });

  // Chat actions
  const newChatBtn = document.getElementById('newChatBtn');
  const clearChatBtn = document.getElementById('clearChatBtn');

  if (newChatBtn) {
    newChatBtn.addEventListener('click', newChat);
  }

  if (clearChatBtn) {
    clearChatBtn.addEventListener('click', clearChat);
  }

  // Settings
  const saveSettingsBtn = document.getElementById('saveSettingsBtn');
  const resetSettingsBtn = document.getElementById('resetSettingsBtn');
  const temperatureInput = document.getElementById('temperatureInput');
  const temperatureValue = document.getElementById('temperatureValue');

  if (saveSettingsBtn) {
    saveSettingsBtn.addEventListener('click', saveSettings);
  }

  if (resetSettingsBtn) {
    resetSettingsBtn.addEventListener('click', resetSettings);
  }

  if (temperatureInput && temperatureValue) {
    temperatureInput.addEventListener('input', () => {
      temperatureValue.textContent = temperatureInput.value;
    });
  }

  // File panel
  const refreshFilesBtn = document.getElementById('refreshFilesBtn');
  const openFolderBtn = document.getElementById('openFolderBtn');

  if (refreshFilesBtn) {
    refreshFilesBtn.addEventListener('click', refreshFiles);
  }

  if (openFolderBtn) {
    openFolderBtn.addEventListener('click', openFolder);
  }

  // Git panel
  const refreshGitBtn = document.getElementById('refreshGitBtn');

  if (refreshGitBtn) {
    refreshGitBtn.addEventListener('click', refreshGit);
  }

  // MCP panel
  const addMcpBtn = document.getElementById('addMcpBtn');

  if (addMcpBtn) {
    addMcpBtn.addEventListener('click', addMcpServer);
  }

  // Project selector
  const currentProject = document.querySelector('.current-project');

  if (currentProject) {
    currentProject.addEventListener('click', openFolder);
  }
}

function switchPanel(tab) {
  // Update nav buttons
  document.querySelectorAll('.nav-btn').forEach(btn => {
    btn.classList.toggle('active', btn.dataset.tab === tab);
  });

  // Update panels
  document.querySelectorAll('.panel').forEach(panel => {
    panel.classList.toggle('active', panel.id === `${tab}Panel`);
  });

  // Load panel content
  if (tab === 'files') refreshFiles();
  if (tab === 'git') refreshGit();
}

async function sendMessage() {
  const chatInput = document.getElementById('chatInput');
  const chatMessages = document.getElementById('chatMessages');

  if (!chatInput || !chatMessages) return;

  const message = chatInput.value.trim();
  if (!message) return;

  // Hide welcome message if visible
  const welcomeMessage = chatMessages.querySelector('.welcome-message');
  if (welcomeMessage) {
    welcomeMessage.style.display = 'none';
  }

  // Add user message
  addMessage('user', message);
  chatInput.value = '';
  autoResizeTextarea(chatInput);

  // Show typing indicator
  showTypingIndicator();

  try {
    // Send to backend
    const response = await invoke('send_message', { message });
    removeTypingIndicator();
    addMessage(response.role, response.content);
  } catch (error) {
    removeTypingIndicator();
    addMessage('assistant', `Error: ${error}`);
  }
}

function addMessage(role, content) {
  const chatMessages = document.getElementById('chatMessages');
  if (!chatMessages) return;

  const messageId = Date.now().toString();
  const messageEl = document.createElement('div');
  messageEl.className = `message ${role}`;
  messageEl.id = `message-${messageId}`;

  const avatar = role === 'user' ? '👤' : '🟣';
  const roleName = role === 'user' ? 'You' : 'Claude';

  messageEl.innerHTML = `
    <div class="message-avatar">${avatar}</div>
    <div class="message-content">
      <div class="message-header">
        <span class="message-role">${roleName}</span>
        <span class="message-time">${new Date().toLocaleTimeString()}</span>
      </div>
      <div class="message-body">${formatMarkdown(content)}</div>
    </div>
  `;

  chatMessages.appendChild(messageEl);
  chatMessages.scrollTop = chatMessages.scrollHeight;
  chatHistory.push({ role, content, id: messageId });
}

function formatMarkdown(content) {
  // Basic markdown formatting
  return content
    .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
    .replace(/\*(.*?)\*/g, '<em>$1</em>')
    .replace(/`([^`]+)`/g, '<code>$1</code>')
    .replace(/^### (.*$)/gim, '<h3>$1</h3>')
    .replace(/^## (.*$)/gim, '<h2>$1</h2>')
    .replace(/^# (.*$)/gim, '<h1>$1</h1>')
    .replace(/\n\n/g, '</p><p>')
    .replace(/^- (.*$)/gim, '<li>$1</li>')
    .replace(/\n/g, '<br>');
}

function showTypingIndicator() {
  const chatMessages = document.getElementById('chatMessages');
  if (!chatMessages) return;

  const typingEl = document.createElement('div');
  typingEl.className = 'message assistant';
  typingEl.id = 'typing-indicator';
  typingEl.innerHTML = `
    <div class="message-avatar">🟣</div>
    <div class="message-content">
      <div class="typing-indicator">
        <div class="typing-dot"></div>
        <div class="typing-dot"></div>
        <div class="typing-dot"></div>
      </div>
    </div>
  `;

  chatMessages.appendChild(typingEl);
  chatMessages.scrollTop = chatMessages.scrollHeight;
}

function removeTypingIndicator() {
  const typingIndicator = document.getElementById('typing-indicator');
  if (typingIndicator) {
    typingIndicator.remove();
  }
}

function handleChatKeydown(event) {
  const chatInput = event.target;

  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault();
    sendMessage();
  } else if (event.key === 'Enter' && event.shiftKey) {
    // Allow newlines with Shift+Enter
  }

  autoResizeTextarea(chatInput);
}

function autoResizeTextarea(textarea) {
  textarea.style.height = 'auto';
  textarea.style.height = Math.min(textarea.scrollHeight, 200) + 'px';
}

function newChat() {
  clearChat();
  const welcomeMessage = document.querySelector('.welcome-message');
  if (welcomeMessage) {
    welcomeMessage.style.display = 'block';
  }
}

async function clearChat() {
  const chatMessages = document.getElementById('chatMessages');
  if (!chatMessages) return;

  chatMessages.innerHTML = `
    <div class="welcome-message">
      <div class="welcome-icon">🟣</div>
      <h1>Hello, I'm Claude</h1>
      <p>I'm your AI coding assistant. I understand your codebase and help you code faster.</p>
      <div class="quick-actions">
        <button class="quick-action" data-prompt="Explain this project">
          <span>📖</span>
          <span>Explain this project</span>
        </button>
        <button class="quick-action" data-prompt="List all files">
          <span>📁</span>
          <span>List all files</span>
        </button>
        <button class="quick-action" data-prompt="Check git status">
          <span>📦</span>
          <span>Check git status</span>
        </button>
        <button class="quick-action" data-prompt="Show recent changes">
          <span>📋</span>
          <span>Show recent changes</span>
        </button>
      </div>
    </div>
  `;

  chatHistory = [];
  await invoke('clear_chat_history').catch(() => {});

  // Re-bind quick actions
  document.querySelectorAll('.quick-action').forEach(btn => {
    btn.addEventListener('click', () => {
      const prompt = btn.dataset.prompt;
      const chatInput = document.getElementById('chatInput');
      if (prompt && chatInput) {
        chatInput.value = prompt;
        sendMessage();
      }
    });
  });
}

async function loadSettings() {
  try {
    const savedSettings = await invoke('get_settings');
    settings = { ...settings, ...savedSettings };
    applySettingsToUI();
  } catch {
    // Use default settings
    applySettingsToUI();
  }
}

function applySettingsToUI() {
  const apiKeyInput = document.getElementById('apiKeyInput');
  const baseUrlInput = document.getElementById('baseUrlInput');
  const modelSelect = document.getElementById('modelSelect');
  const maxTokensInput = document.getElementById('maxTokensInput');
  const temperatureInput = document.getElementById('temperatureInput');
  const temperatureValue = document.getElementById('temperatureValue');
  const themeSelect = document.getElementById('themeSelect');

  if (apiKeyInput) apiKeyInput.value = settings.apiKey;
  if (baseUrlInput) baseUrlInput.value = settings.baseUrl;
  if (modelSelect) modelSelect.value = settings.model;
  if (maxTokensInput) maxTokensInput.value = settings.maxTokens;
  if (temperatureInput) temperatureInput.value = settings.temperature;
  if (temperatureValue) temperatureValue.textContent = settings.temperature;
  if (themeSelect) themeSelect.value = settings.theme;
}

async function saveSettings() {
  const apiKeyInput = document.getElementById('apiKeyInput');
  const baseUrlInput = document.getElementById('baseUrlInput');
  const modelSelect = document.getElementById('modelSelect');
  const maxTokensInput = document.getElementById('maxTokensInput');
  const temperatureInput = document.getElementById('temperatureInput');
  const themeSelect = document.getElementById('themeSelect');

  settings = {
    apiKey: apiKeyInput?.value || '',
    baseUrl: baseUrlInput?.value || 'https://api.anthropic.com',
    model: modelSelect?.value || 'claude-3-opus-20240229',
    theme: themeSelect?.value || 'dark',
    autoSave: true,
    maxTokens: parseInt(maxTokensInput?.value) || 4096,
    temperature: parseFloat(temperatureInput?.value) || 0.7
  };

  try {
    await invoke('save_settings', { settings });
    showNotification('Settings saved successfully', 'success');
  } catch (error) {
    showNotification(`Error saving settings: ${error}`, 'error');
  }
}

function resetSettings() {
  settings = {
    apiKey: '',
    baseUrl: 'https://api.anthropic.com',
    model: 'claude-3-opus-20240229',
    theme: 'dark',
    autoSave: true,
    maxTokens: 4096,
    temperature: 0.7
  };
  applySettingsToUI();
  showNotification('Settings reset to defaults', 'info');
}

async function loadCurrentDirectory() {
  try {
    currentDirectory = await invoke('get_current_directory');
    updateDirectoryDisplay();
  } catch {
    // Use default
  }
}

function updateDirectoryDisplay() {
  const projectName = document.getElementById('projectName');
  const projectPath = document.getElementById('projectPath');

  if (projectName && projectPath) {
    const pathParts = currentDirectory.split(/[/\\]/);
    const name = pathParts[pathParts.length - 1] || 'No Project';
    projectName.textContent = name;
    projectPath.textContent = currentDirectory;
  }
}

async function openFolder() {
  try {
    const path = await invoke('dialog', { type: 'folder' });
    if (path) {
      await invoke('set_current_directory', { path });
      currentDirectory = path;
      updateDirectoryDisplay();
      refreshFiles();
    }
  } catch (error) {
    console.error('Failed to open folder:', error);
  }
}

async function refreshFiles() {
  const fileTree = document.getElementById('fileTree');
  if (!fileTree) return;

  try {
    const files = await invoke('list_directory', { path: currentDirectory });
    renderFileTree(files);
  } catch {
    fileTree.innerHTML = `
      <div class="empty-state">
        <span>📂</span>
        <p>Failed to load files</p>
      </div>
    `;
  }
}

function renderFileTree(files) {
  const fileTree = document.getElementById('fileTree');
  if (!fileTree) return;

  if (files.length === 0) {
    fileTree.innerHTML = `
      <div class="empty-state">
        <span>📂</span>
        <p>No files in this directory</p>
      </div>
    `;
    return;
  }

  const sortedFiles = [...files].sort((a, b) => {
    if (a.is_dir && !b.is_dir) return -1;
    if (!a.is_dir && b.is_dir) return 1;
    return a.name.localeCompare(b.name);
  });

  fileTree.innerHTML = sortedFiles.map(file => `
    <div class="file-item" data-path="${file.path}">
      <span class="file-icon">${file.is_dir ? '📁' : '📄'}</span>
      <span class="file-name">${file.name}</span>
      ${file.size ? `<span class="file-meta">${formatFileSize(file.size)}</span>` : ''}
    </div>
  `).join('');

  // Add click handlers
  fileTree.querySelectorAll('.file-item').forEach(item => {
    item.addEventListener('click', () => handleFileClick(item.dataset.path));
  });
}

function handleFileClick(path) {
  console.log('File clicked:', path);
}

function formatFileSize(bytes) {
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
}

async function refreshGit() {
  const gitContent = document.getElementById('gitContent');
  if (!gitContent) return;

  gitContent.innerHTML = `
    <div class="empty-state">
      <span>📦</span>
      <p>Git status coming soon...</p>
    </div>
  `;
}

async function addMcpServer() {
  showNotification('MCP server configuration coming soon', 'info');
}

function showNotification(message, type = 'info') {
  console.log(`[${type.toUpperCase()}]`, message);
}

console.log('Claude Code GUI initialized');
