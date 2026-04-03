import { test, expect } from '@playwright/test';

test.describe('Serial Port Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:1420');
    // Wait for app to load
    await page.waitForSelector('text=串口调试');
  });

  test('should display serial page', async ({ page }) => {
    await expect(page.locator('text=串口调试')).toBeVisible();
    await expect(page.locator('text=连接')).toBeVisible();
  });

  test('should refresh ports list', async ({ page }) => {
    const refreshButton = page.locator('[title="刷新串口列表"]');
    await refreshButton.click();
    
    // Wait for the refresh operation
    await page.waitForTimeout(500);
    
    // Check if port dropdown is accessible
    await expect(page.locator('text=串口')).toBeVisible();
  });

  test('should switch input modes', async ({ page }) => {
    // Find and click the hex mode button
    const hexButton = page.locator('button[value="hex"]');
    await hexButton.click();
    
    // Check if input placeholder changes
    const input = page.locator('input[placeholder*="十六进制"]');
    await expect(input).toBeVisible();
  });

  test('should show disabled state when not connected', async ({ page }) => {
    // Send button should be disabled
    const sendButton = page.locator('button:has-text("发送")');
    await expect(sendButton).toBeDisabled();
  });
});